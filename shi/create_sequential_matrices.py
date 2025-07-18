#!/usr/bin/env python3
"""
Последовательное создание матриц расстояний для разного количества звезд
"""

import json
import numpy as np
import os
import time
from typing import List, Dict, Any
from star_matrix_processor import StarMatrixProcessor
from astroquery.gaia import Gaia


def test_connection():
    """Тест подключения к Gaia Archive"""
    print("🔍 Тест подключения к Gaia Archive...")

    try:
        query = "SELECT TOP 1 source_id FROM gaiadr3.gaia_source WHERE parallax > 0"

        Gaia.ROW_LIMIT = 1
        Gaia.TIMEOUT = 30

        print("📡 Отправка запроса...")
        job = Gaia.launch_job_async(query)

        print("⏳ Ожидание ответа...")
        results = job.get_results()

        if len(results) > 0:
            print("✅ Подключение успешно!")
            return True
        else:
            print("❌ Нет данных в ответе")
            return False

    except Exception as e:
        print(f"❌ Ошибка: {e}")
        return False


def fetch_stars_and_save(limit: int) -> bool:
    """Получение звезд и немедленное сохранение матрицы"""
    print(f"\n🔍 Получение {limit} звезд...")

    query = f"""
    SELECT TOP {limit}
        source_id,
        ra,
        dec,
        parallax,
        pmra,
        pmdec,
        phot_g_mean_mag,
        bp_rp
    FROM gaiadr3.gaia_source
    WHERE parallax > 0
    AND parallax_error/parallax < 0.3
    AND phot_g_mean_mag IS NOT NULL
    AND phot_g_mean_mag < 20
    ORDER BY parallax DESC
    """

    try:
        print("📡 Отправка запроса...")

        Gaia.ROW_LIMIT = limit
        Gaia.TIMEOUT = 120

        job = Gaia.launch_job_async(query)

        print("⏳ Ожидание результатов...")
        results = job.get_results()

        if len(results) > 0:
            stars = []

            # Функция для безопасного преобразования значений
            def safe_float(value):
                if value is None or (hasattr(value, 'mask') and value.mask):
                    return None
                try:
                    return float(value)
                except (ValueError, TypeError):
                    return None

            for row in results:
                star = {
                    'source_id': int(row['source_id']),
                    'ra': float(row['ra']),
                    'dec': float(row['dec']),
                    'parallax': float(row['parallax']),
                    'pmra': safe_float(row['pmra']) or 0.0,
                    'pmdec': safe_float(row['pmdec']) or 0.0,
                    'phot_g_mean_mag': safe_float(row['phot_g_mean_mag']),
                    'bp_rp': safe_float(row['bp_rp'])
                }
                stars.append(star)

            print(f"✅ Получено {len(stars)} звезд")

            # Немедленно создаем и сохраняем матрицу
            return create_and_save_matrix(stars, limit)
        else:
            print("❌ Нет данных в ответе")
            return False

    except Exception as e:
        print(f"❌ Ошибка запроса: {e}")
        return False


def create_and_save_matrix(stars_data: List[Dict[str, Any]], limit: int) -> bool:
    """Создание и сохранение матрицы для заданного количества звезд"""
    print(f"⚙️ Обработка {len(stars_data)} звезд...")

    try:
        processor = StarMatrixProcessor()
        processed_stars = processor.process_star_data(stars_data)

        if not processed_stars:
            print("❌ Ошибка обработки данных")
            return False

        matrix = processor.create_distance_matrix(processed_stars)

        if matrix.size == 0:
            print("❌ Ошибка создания матрицы")
            return False

        print(f"📊 Создана матрица размером {matrix.shape}")
        print(f"📈 Диапазон расстояний: {np.min(matrix):.2f} - {np.max(matrix):.2f} парсек")

        processor.save_matrix_as_tsp(matrix, "best.txt")

        # Сохраняем форматированный файл
        formatted_file = f"stars_{limit}_formatted.txt"
        with open(formatted_file, 'w', encoding='utf-8') as f:
            f.write(f"{len(processed_stars)}\n")

            for i in range(len(processed_stars)):
                row = []
                for j in range(len(processed_stars)):
                    if i == j:
                        row.append("-1")
                    else:
                        distance = int(round(matrix[i][j]))
                        row.append(str(distance))
                f.write(", ".join(row) + "\n")

        print(f"✅ Форматированная матрица сохранена в {formatted_file}")

        # Сохраняем сырой файл с точными значениями
        raw_file = f"stars_{limit}_raw.txt"
        with open(raw_file, 'w', encoding='utf-8') as f:
            f.write(f"# Матрица расстояний между {len(processed_stars)} звездами\n")
            f.write(f"# Размер матрицы: {len(processed_stars)}x{len(processed_stars)}\n")
            f.write(f"# Единицы измерения: парсеки\n")
            f.write(f"# Диапазон: {np.min(matrix):.6f} - {np.max(matrix):.6f}\n")
            f.write(f"# Среднее расстояние: {np.mean(matrix):.6f}\n")
            f.write(f"# Стандартное отклонение: {np.std(matrix):.6f}\n")
            f.write("#\n")

            for i in range(len(processed_stars)):
                row = []
                for j in range(len(processed_stars)):
                    if i == j:
                        row.append("0.000000")
                    else:
                        distance = matrix[i][j]
                        row.append(f"{distance:.6f}")
                f.write(" ".join(row) + "\n")

        print(f"✅ Сырая матрица сохранена в {raw_file}")
        print(f"📊 Размер: {len(processed_stars)}x{len(processed_stars)}")

        return True

    except Exception as e:
        print(f"❌ Ошибка создания матрицы: {e}")
        return False


def main():
    """Главная функция - последовательное создание матриц"""
    print("🚀 ПОСЛЕДОВАТЕЛЬНОЕ СОЗДАНИЕ МАТРИЦ РАССТОЯНИЙ")
    print("Создаем матрицы для 1000, 2000, 3000, 4000, 5000 звезд")
    print("=" * 60)

    # Тестируем подключение
    if not test_connection():
        print("❌ Не удалось подключиться к Gaia Archive")
        print("💡 Попробуйте позже или проверьте интернет соединение")
        return

    # Последовательное создание матриц
    limits = [10000]

    for limit in limits:
        print(f"\n{'='*50}")
        print(f"🎯 ОБРАБОТКА {limit} ЗВЕЗД")
        print(f"{'='*50}")

        success = fetch_stars_and_save(limit)

        if success:
            print(f"✅ Матрица для {limit} звезд создана успешно!")
        else:
            print(f"❌ Ошибка при создании матрицы для {limit} звезд")
            print("🔄 Переходим к следующему количеству...")

        # Пауза между запросами
        if limit < limits[-1]:  # Если это не последний элемент
            print(f"\n⏳ Пауза 30 секунд перед следующим запросом...")
            time.sleep(30)

    print(f"\n{'='*60}")
    print("🎉 ЗАВЕРШЕНО СОЗДАНИЕ ВСЕХ МАТРИЦ")
    print("📁 Созданные файлы:")
    for limit in limits:
        print(f"   - stars_{limit}_formatted.txt")
        print(f"   - stars_{limit}_raw.txt")
    print(f"{'='*60}")


if __name__ == "__main__":
    main()
