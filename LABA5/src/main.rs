use anyhow::Result as AnyHowResult;
use polars::io::SerReader;
use polars::prelude::{col, CsvReadOptions, IntoLazy};

fn main() -> AnyHowResult<()> {
    // Задание 1.
    // Прочтите исходный файл и выведите первые 10 строк.
    println!("Задание 1.");
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("music_log_2.csv".into()))?
        .finish()?;
    println!("{}", df.head(Some(10)));
    println!("\n");

    // Задание 2.
    // Получите список названий столбцов. Результат выведите на экран.
    println!("Задание 2.");
    let columns = df.get_column_names();
    println!("{:?}", columns);
    println!("\n");

    // Задание 3.
    // Посчитайте количество пропущенных значений.
    println!("Задание 3.");
    let null_count_mask = df.null_count();
    println!("Количество пропущенных значений: {:?}", null_count_mask);
    println!("\n");

    // Задание 4.
    // Посчитайте количество дубликатов в наборе данных.
    println!("Задание 4.");
    let duplicates_count = df.is_duplicated()?.sum().unwrap();
    println!("Количество дубликатов: {:?}", duplicates_count);
    println!("\n");

    // Задание 5.
    // Сгруппируйте DataFrame по столбцу user_id.
    // Посчитайте количество жанров, которые выбрал каждый пользователь, используя столбец genre_name.
    // Сохраните результат группировки и подсчёта в переменных genre_grouping и genre_counting, соответственно.
    // Выведите первые 30 строк результата.
    let genre_counting = df
        .lazy()
        .group_by([col("user_id")])
        .agg([col("genre_name").count().alias("genre_count")])
        .sort(["genre_count"], Default::default())
        .collect()?;

    // Выводим первые 30 строк результата
    println!("{:?}", genre_counting.head(Some(30)));

    Ok(())
}
