#!/usr/bin/env python3
"""
Обработчик звездной матрицы для работы с данными Gaia
"""

import json
import numpy as np
import os
from typing import List, Dict, Any, Tuple
from pathlib import Path


class StarMatrixProcessor:
    """Обработчик для создания матрицы евклидовых расстояний между звездами"""

    def __init__(self):
        self.processed_stars = []
        self.distance_matrix = None

    def load_star_data(self, file_path: str) -> List[Dict[str, Any]]:
        """Загрузка данных звезд из JSON файла"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                data = json.load(f)

            if not isinstance(data, list):
                raise ValueError("Данные должны быть списком звезд")

            print(f"✅ Загружено {len(data)} звезд из {file_path}")
            return data

        except FileNotFoundError:
            print(f"❌ Файл {file_path} не найден")
            return []
        except json.JSONDecodeError as e:
            print(f"❌ Ошибка парсинга JSON: {e}")
            return []
        except Exception as e:
            print(f"❌ Ошибка загрузки данных: {e}")
            return []

    def process_star_data(self, star_data: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Обработка данных звезд и вычисление расстояний"""
        if not star_data:
            print("❌ Нет данных для обработки")
            return []

        processed_stars = []
        valid_count = 0

        for star in star_data:
            try:
                # Проверяем наличие необходимых полей
                required_fields = ['source_id', 'ra', 'dec', 'parallax']
                if not all(field in star for field in required_fields):
                    continue

                # Получаем данные
                source_id = star['source_id']
                ra = float(star['ra'])  # прямое восхождение в градусах
                dec = float(star['dec'])  # склонение в градусах
                parallax = float(star['parallax'])  # параллакс в миллисекундах дуги

                # Проверяем валидность параллакса
                if parallax <= 0:
                    continue

                # Вычисляем расстояние в парсеках
                distance = 1000.0 / parallax  # формула: d = 1/p (пк)

                # Преобразуем координаты в радианы
                ra_rad = np.radians(ra)
                dec_rad = np.radians(dec)

                # Вычисляем декартовые координаты
                x = distance * np.cos(dec_rad) * np.cos(ra_rad)
                y = distance * np.cos(dec_rad) * np.sin(ra_rad)
                z = distance * np.sin(dec_rad)

                # Вычисляем собственную скорость (если есть данные)
                velocity = 0.0
                if 'pmra' in star and 'pmdec' in star:
                    try:
                        pmra = float(star['pmra'])  # собственное движение по RA (мс/год)
                        pmdec = float(star['pmdec'])  # собственное движение по Dec (мс/год)

                        # Простое вычисление скорости (упрощенное)
                        velocity = np.sqrt(pmra**2 + pmdec**2)
                    except (ValueError, TypeError):
                        velocity = 0.0

                # Создаем обработанную запись
                processed_star = {
                    'source_id': source_id,
                    'ra': ra,
                    'dec': dec,
                    'parallax': parallax,
                    'distance': distance,
                    'x': x,
                    'y': y,
                    'z': z,
                    'velocity': velocity
                }

                processed_stars.append(processed_star)
                valid_count += 1

            except (ValueError, TypeError) as e:
                print(f"⚠️ Ошибка обработки звезды {star.get('source_id', 'unknown')}: {e}")
                continue

        self.processed_stars = processed_stars
        print(f"✅ Обработано {valid_count} звезд из {len(star_data)}")

        if valid_count == 0:
            print("❌ Нет валидных звезд после обработки")

        return processed_stars


    def create_distance_matrix(self, processed_stars: List[Dict[str, Any]], chunk_size=500) -> np.ndarray:
        if not processed_stars:
            print("❌ Нет обработанных звезд для создания матрицы")
            return np.array([])

        coords = np.array([[star['x'], star['y'], star['z']] for star in processed_stars])
        n = coords.shape[0]
        matrix = np.zeros((n, n), dtype=np.float32)

        print(f"📐 Расчёт расстояний для {n} звезд чанками по {chunk_size}...")

        for i in range(0, n, chunk_size):
            i_end = min(i + chunk_size, n)
            ci = coords[i:i_end]

        # Use broadcasting to compute distances to all stars
            diff = ci[:, np.newaxis, :] - coords[np.newaxis, :, :]
            dists = np.linalg.norm(diff, axis=-1)

            matrix[i:i_end] = dists

        self.distance_matrix = matrix

        non_zero = matrix[matrix > 0]
        print(f"✅ Матрица создана: {matrix.shape[0]}x{matrix.shape[1]}")
        if non_zero.size > 0:
            print(f"   - Мин. расстояние: {np.min(non_zero):.2f} пк")
            print(f"   - Макс. расстояние: {np.max(non_zero):.2f} пк")
            print(f"   - Среднее расстояние: {np.mean(non_zero):.2f} пк")
        else:
            print("⚠️ Матрица пустая")

        return matrix
    def sort_stars_by_distance(self, processed_stars: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Сортировка звезд по расстоянию (от ближайших к дальним)"""
        if not processed_stars:
            return []

        sorted_stars = sorted(processed_stars, key=lambda x: x['distance'])

        print(f"📊 Звезды отсортированы по расстоянию:")
        print(f"   - Ближайшая: {sorted_stars[0]['distance']:.2f} пк")
        print(f"   - Дальняя: {sorted_stars[-1]['distance']:.2f} пк")

        return sorted_stars

    def save_processed_data(self, processed_stars: List[Dict[str, Any]], output_file: str):
        """Сохранение обработанных данных в JSON файл"""
        try:
            with open(output_file, 'w', encoding='utf-8') as f:
                json.dump(processed_stars, f, indent=2, ensure_ascii=False)

            print(f"✅ Обработанные данные сохранены в {output_file}")

        except Exception as e:
            print(f"❌ Ошибка сохранения данных: {e}")


    def save_matrix_to_file(self, matrix: np.ndarray, output_file: str):
        """Сохраняем координаты звезд как float"""
        try:
            with open(output_file, 'w') as f:

            # Сохраняем реальные координаты звезд
                for i, star in enumerate(self.processed_stars):
                    f.write(f"{i+1} {star['x']:.6f} {star['y']:.6f}\n")

                f.write("EOF\n")

            print(f"✅ Матрица сохранена в {output_file}")
        except Exception as e:
            print(f"❌ Ошибка сохранения матрицы: {e}")


    def save_matrix_as_tsp(self, matrix: np.ndarray, output_file: str):
        """Сохраняет матрицу расстояний в формате TSPLIB (FULL_MATRIX)"""
        try:
            n = matrix.shape[0]

            with open(output_file, 'w') as f:
                f.write("NAME: star_distance_tsp\n")
                f.write("TYPE: TSP\n")
                f.write(f"DIMENSION: {n}\n")
                f.write("EDGE_WEIGHT_TYPE: EXPLICIT\n")
                f.write("EDGE_WEIGHT_FORMAT: FULL_MATRIX\n")
                f.write("EDGE_WEIGHT_SECTION\n")

                for row in matrix:
                    line = ' '.join(f"{int(round(val))}" for val in row)
                    f.write(line + "\n")

                f.write("EOF\n")

            print(f"✅ Матрица TSP сохранена в {output_file}")
        except Exception as e:
            print(f"❌ Ошибка при сохранении TSP-матрицы: {e}")


    def print_statistics(self, processed_stars: List[Dict[str, Any]]):
        """Вывод статистики по обработанным звездам"""
        if not processed_stars:
            print("❌ Нет данных для статистики")
            return

        distances = [star['distance'] for star in processed_stars]
        velocities = [star['velocity'] for star in processed_stars]

        print("\n📊 СТАТИСТИКА ОБРАБОТАННЫХ ЗВЕЗД:")
        print(f"   - Всего звезд: {len(processed_stars)}")
        print(f"   - Среднее расстояние: {np.mean(distances):.2f} пк")
        print(f"   - Мин. расстояние: {np.min(distances):.2f} пк")
        print(f"   - Макс. расстояние: {np.max(distances):.2f} пк")
        print(f"   - Средняя скорость: {np.mean(velocities):.2f} мс/год")

    def process_file(self, input_file: str, output_dir: str = "output"):
        """Полный процесс обработки файла"""
        print(f"🚀 Начинаем обработку файла: {input_file}")

        # Создаем выходную директорию
        os.makedirs(output_dir, exist_ok=True)

        # Загружаем данные
        star_data = self.load_star_data(input_file)
        if not star_data:
            return False

        # Обрабатываем данные
        processed_stars = self.process_star_data(star_data)
        if not processed_stars:
            return False

        # Сортируем звезды
        sorted_stars = self.sort_stars_by_distance(processed_stars)

        # Создаем матрицу расстояний
        matrix = self.create_distance_matrix(sorted_stars)
        if matrix.size == 0:
            return False

        # Сохраняем результаты
        output_file = os.path.join(output_dir, "processed_stars.json")
        self.save_processed_data(sorted_stars, output_file)

        matrix_file = os.path.join(output_dir, "distance_matrix.txt")
        self.save_matrix_to_file(matrix, matrix_file)

        # Выводим статистику
        self.print_statistics(sorted_stars)

        print(f"✅ Обработка завершена. Результаты сохранены в {output_dir}/")
        return True


if __name__ == "__main__":
    # Пример использования
    processor = StarMatrixProcessor()

    # Обрабатываем тестовый файл
    test_file = "tests/test_stars_small.json"
    if os.path.exists(test_file):
        processor.process_file(test_file)
    else:
        print(f"❌ Тестовый файл {test_file} не найден")
