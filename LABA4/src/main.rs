use anyhow::Result as AnyHowResult;
use polars::datatypes::PlSmallStr;
use polars::io::SerReader;
use polars::prelude::{CsvReadOptions, SchemaExt};

fn main() -> AnyHowResult<()> {
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("music_log.csv".into()))?
        .finish()?;
    println!("{:?}", df);

    // Задание 1.
    // Просмотрите информацию о наборе данных.
    println!("Задание 1.");
    df.schema().iter_fields().for_each(|field| {
        println!("{:?}", field);
    });
    println!("\n");

    // Задание 2.
    // Выведите список столбцов.
    println!("Задание 2.");
    let columns = df.get_column_names();
    println!("{:?}", columns);
    println!("\n");

    // Задание 3.
    // Подготовьте список new_names с новыми именами для столбцов.
    // • user_id → user_id
    // • total play → total_play_seconds
    // • Artist → artist_name
    // • genre → genre_name
    // • track → track_name
    let new_names = vec![
        "user_id",
        "total_play_seconds",
        "artist_name",
        "genre_name",
        "track_name",
    ];
    println!("\n");

    // Задание 4.
    // Переименуйте столбцы таблицы.
    let mut df_renamed = df.clone();
    if df_renamed.get_column_names().contains(&&PlSmallStr::from("_duplicated_0")) &&
        df_renamed.get_column_names().contains(&&PlSmallStr::from("")) {
        df_renamed = df_renamed.drop("_duplicated_0")?;
        df_renamed = df_renamed.drop("")?;
    }
    df_renamed.set_column_names(new_names)?;
    println!("\n");

    // Задание 5.
    // Проверьте, что получилось, запросив для нашей структуры данных df атрибут columns.
    println!("Задание 5.");
    println!("{:?}", df_renamed.get_column_names());
    println!("\n");

    // Задание 6.
    // Просмотрите информацию о наборе данных
    println!("Задание 6.");
    df_renamed.schema().iter_fields().for_each(|field| {
        println!("{:?}", field);
    });
    println!("\n");

    // Задание 7.
    // Посчитайте количество пропущенных значений и выведите его на экран.
    println!("Задание 7.");
    let missing_values = df_renamed.null_count();

    println!("Количество пропущенных значений: {}", missing_values);




    Ok(())
}
