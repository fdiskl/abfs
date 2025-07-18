#!/usr/bin/env python3
"""
Валидатор данных звезд для проверки качества и структуры
"""

import json
import numpy as np
import os
from typing import Dict, List, Any, Tuple
from pathlib import Path


class DataValidator:
    """Валидатор для проверки качества данных звезд"""
    
    def __init__(self):
        self.validation_results = {}
        
    def validate_star_data(self, star_data: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Валидация данных звезд"""
        if not star_data:
            return {
                'is_valid': False,
                'errors': ['Пустые данные'],
                'star_count': 0
            }
        
        errors = []
        warnings = []
        
        # Проверяем структуру данных
        if not isinstance(star_data, list):
            errors.append("Данные должны быть списком")
            return {
                'is_valid': False,
                'errors': errors,
                'star_count': 0
            }
        
        # Проверяем каждую звезду
        valid_stars = 0
        parallax_count = 0
        pm_count = 0
        
        required_fields = ['source_id', 'ra', 'dec', 'parallax']
        
        for i, star in enumerate(star_data):
            if not isinstance(star, dict):
                errors.append(f"Звезда {i} не является словарем")
                continue
            
            # Проверяем обязательные поля
            missing_fields = [field for field in required_fields if field not in star]
            if missing_fields:
                errors.append(f"Звезда {i} (ID: {star.get('source_id', 'unknown')}): отсутствуют поля {missing_fields}")
                continue
            
            # Проверяем типы данных
            try:
                source_id = star['source_id']
                ra = float(star['ra'])
                dec = float(star['dec'])
                parallax = float(star['parallax'])
                
                # Проверяем диапазоны
                if not (0 <= ra <= 360):
                    warnings.append(f"Звезда {i}: RA вне диапазона [0, 360]: {ra}")
                
                if not (-90 <= dec <= 90):
                    warnings.append(f"Звезда {i}: Dec вне диапазона [-90, 90]: {dec}")
                
                if parallax <= 0:
                    errors.append(f"Звезда {i}: невалидный параллакс: {parallax}")
                    continue
                else:
                    parallax_count += 1
                
                # Проверяем собственные движения
                if 'pmra' in star and 'pmdec' in star:
                    try:
                        pmra = float(star['pmra'])
                        pmdec = float(star['pmdec'])
                        pm_count += 1
                    except (ValueError, TypeError):
                        warnings.append(f"Звезда {i}: невалидные собственные движения")
                
                valid_stars += 1
                
            except (ValueError, TypeError) as e:
                errors.append(f"Звезда {i}: ошибка типов данных: {e}")
                continue
        
        # Проверяем общую статистику
        if valid_stars == 0:
            errors.append("Нет валидных звезд")
        
        if valid_stars > 0 and valid_stars < len(star_data) * 0.5:
            warnings.append(f"Много невалидных звезд: {valid_stars}/{len(star_data)}")
        
        # Формируем результат
        result = {
            'is_valid': len(errors) == 0,
            'errors': errors,
            'warnings': warnings,
            'star_count': valid_stars,
            'total_count': len(star_data),
            'parallax_count': parallax_count,
            'pm_count': pm_count,
            'validity_rate': valid_stars / len(star_data) if star_data else 0
        }
        
        self.validation_results = result
        return result
    
    def validate_processed_data(self, processed_stars: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Валидация обработанных данных звезд"""
        if not processed_stars:
            return {
                'is_valid': False,
                'errors': ['Пустые обработанные данные'],
                'star_count': 0
            }
        
        errors = []
        warnings = []
        
        # Проверяем структуру обработанных данных
        required_fields = ['source_id', 'distance', 'x', 'y', 'z']
        
        valid_stars = 0
        distances = []
        
        for i, star in enumerate(processed_stars):
            if not isinstance(star, dict):
                errors.append(f"Обработанная звезда {i} не является словарем")
                continue
            
            # Проверяем обязательные поля
            missing_fields = [field for field in required_fields if field not in star]
            if missing_fields:
                errors.append(f"Обработанная звезда {i}: отсутствуют поля {missing_fields}")
                continue
            
            try:
                source_id = star['source_id']
                distance = float(star['distance'])
                x = float(star['x'])
                y = float(star['y'])
                z = float(star['z'])
                
                # Проверяем валидность расстояния
                if distance <= 0:
                    errors.append(f"Звезда {i}: невалидное расстояние: {distance}")
                    continue
                
                distances.append(distance)
                valid_stars += 1
                
                # Проверяем координаты
                if not np.isfinite(x) or not np.isfinite(y) or not np.isfinite(z):
                    warnings.append(f"Звезда {i}: невалидные координаты")
                
            except (ValueError, TypeError) as e:
                errors.append(f"Обработанная звезда {i}: ошибка типов данных: {e}")
                continue
        
        # Анализируем статистику расстояний
        if distances:
            min_dist = min(distances)
            max_dist = max(distances)
            mean_dist = np.mean(distances)
            
            if max_dist > 1000:  # Более 1000 пк
                warnings.append(f"Очень большие расстояния: до {max_dist:.1f} пк")
            
            if min_dist < 0.1:  # Менее 0.1 пк
                warnings.append(f"Очень малые расстояния: от {min_dist:.3f} пк")
        
        result = {
            'is_valid': len(errors) == 0,
            'errors': errors,
            'warnings': warnings,
            'star_count': valid_stars,
            'total_count': len(processed_stars),
            'distance_stats': {
                'min': min(distances) if distances else 0,
                'max': max(distances) if distances else 0,
                'mean': np.mean(distances) if distances else 0,
                'std': np.std(distances) if distances else 0
            }
        }
        
        return result
    
    def validate_matrix(self, matrix: np.ndarray) -> Dict[str, Any]:
        """Валидация матрицы расстояний"""
        if matrix is None or matrix.size == 0:
            return {
                'is_valid': False,
                'errors': ['Пустая матрица'],
                'matrix_size': 0
            }
        
        errors = []
        warnings = []
        
        # Проверяем размеры
        if matrix.ndim != 2:
            errors.append("Матрица должна быть двумерной")
            return {
                'is_valid': False,
                'errors': errors,
                'matrix_size': 0
            }
        
        if matrix.shape[0] != matrix.shape[1]:
            errors.append("Матрица должна быть квадратной")
        
        # Проверяем симметричность
        if not np.allclose(matrix, matrix.T):
            errors.append("Матрица не симметрична")
        
        # Проверяем диагональ
        if not np.allclose(np.diag(matrix), 0):
            warnings.append("Диагональ матрицы не содержит нули")
        
        # Проверяем диапазон значений
        non_zero_distances = matrix[matrix > 0]
        if len(non_zero_distances) > 0:
            min_dist = np.min(non_zero_distances)
            max_dist = np.max(non_zero_distances)
            
            if max_dist > 1000:
                warnings.append(f"Очень большие расстояния в матрице: до {max_dist:.1f}")
            
            if min_dist < 0.01:
                warnings.append(f"Очень малые расстояния в матрице: от {min_dist:.3f}")
        
        result = {
            'is_valid': len(errors) == 0,
            'errors': errors,
            'warnings': warnings,
            'matrix_size': matrix.shape[0],
            'distance_stats': {
                'min': np.min(non_zero_distances) if len(non_zero_distances) > 0 else 0,
                'max': np.max(non_zero_distances) if len(non_zero_distances) > 0 else 0,
                'mean': np.mean(non_zero_distances) if len(non_zero_distances) > 0 else 0
            }
        }
        
        return result
    
    def print_validation_report(self, validation_result: Dict[str, Any]):
        """Вывод отчета о валидации"""
        print("\n📋 ОТЧЕТ О ВАЛИДАЦИИ")
        print("=" * 50)
        
        if validation_result['is_valid']:
            print("✅ Данные валидны")
        else:
            print("❌ Данные невалидны")
        
        print(f"📊 Статистика:")
        if 'star_count' in validation_result:
            print(f"   - Валидных звезд: {validation_result['star_count']}")
            if 'total_count' in validation_result:
                print(f"   - Всего звезд: {validation_result['total_count']}")
                rate = validation_result['star_count'] / validation_result['total_count'] * 100
                print(f"   - Процент валидных: {rate:.1f}%")
        
        if 'parallax_count' in validation_result:
            print(f"   - Звезд с параллаксом: {validation_result['parallax_count']}")
        
        if 'pm_count' in validation_result:
            print(f"   - Звезд с собственным движением: {validation_result['pm_count']}")
        
        if 'matrix_size' in validation_result:
            print(f"   - Размер матрицы: {validation_result['matrix_size']}x{validation_result['matrix_size']}")
        
        # Выводим ошибки
        if validation_result['errors']:
            print(f"\n❌ Ошибки ({len(validation_result['errors'])}):")
            for error in validation_result['errors']:
                print(f"   - {error}")
        
        # Выводим предупреждения
        if validation_result['warnings']:
            print(f"\n⚠️ Предупреждения ({len(validation_result['warnings'])}):")
            for warning in validation_result['warnings']:
                print(f"   - {warning}")
        
        # Выводим статистику расстояний
        if 'distance_stats' in validation_result:
            stats = validation_result['distance_stats']
            print(f"\n📏 Статистика расстояний:")
            print(f"   - Мин. расстояние: {stats['min']:.2f} пк")
            print(f"   - Макс. расстояние: {stats['max']:.2f} пк")
            print(f"   - Среднее расстояние: {stats['mean']:.2f} пк")
            if 'std' in stats:
                print(f"   - Стандартное отклонение: {stats['std']:.2f} пк")


def main():
    """Главная функция для тестирования валидатора"""
    validator = DataValidator()
    
    # Тестируем на тестовых данных
    test_files = [
        "tests/test_stars_small.json",
        "tests/test_processed_stars.json"
    ]
    
    for test_file in test_files:
        if os.path.exists(test_file):
            print(f"\n🔍 Валидация файла: {test_file}")
            
            try:
                with open(test_file, 'r') as f:
                    data = json.load(f)
                
                if isinstance(data, list) and len(data) > 0:
                    # Определяем тип данных по структуре
                    if 'distance' in data[0]:
                        # Это обработанные данные
                        result = validator.validate_processed_data(data)
                    else:
                        # Это сырые данные
                        result = validator.validate_star_data(data)
                    
                    validator.print_validation_report(result)
                else:
                    print("❌ Неверный формат данных")
                    
            except Exception as e:
                print(f"❌ Ошибка валидации: {e}")
        else:
            print(f"⚠️ Файл {test_file} не найден")


if __name__ == "__main__":
    main() 