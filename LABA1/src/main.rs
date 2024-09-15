use anyhow::Result as AnyHowResult;
use itertools::Itertools;
use std::iter::Sum;
use std::ops::{Add, Div};

// Количество эмодзи в EmojiXpress(млн)
const TOTAL_EMOJI_IN_EMOJIXPRESS: f64 = 1720.0;
// Количество эмодзи в Instagram(млн)
const TOTAL_EMOJI_IN_TWITTER: f64 = 24500.0;

trait IteratorExt: Iterator {
    fn avg<T>(self) -> Option<T>
    where
        Self: Sized,
        Self::Item: Into<T>,
        T: Sum + Add<Output = T> + Div<Output = T> + From<u8> + Copy,
    {
        let (sum, count): (T, i8) = self.fold((T::from(0), 0), |(sum, count), item| {
            (sum + item.into(), count + 1)
        });

        if count.is_positive() {
            Some(sum / T::from(count as u8))
        } else {
            None
        }
    }
}

impl<I: Iterator> IteratorExt for I {}

fn sort_by_column<T: PartialOrd, U>(data: &[U], column_selector: impl Fn(&U) -> T) -> Vec<U>
where
    U: Clone,
{
    data.iter()
        .sorted_by(|a, b| {
            column_selector(b)
                .partial_cmp(&column_selector(a))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
        .collect()
}

fn main() -> AnyHowResult<()> {
    let names_ru = [
        "Ухмыляюсь",
        "Сияю от радости",
        "Катаюсь от смеха",
        "Слёзы радости",
        "Подмигиваю",
        "Счастлив",
        "Глаза-сердца",
        "Целую",
        "Задумчивость",
        "Равнодушие",
        "Солнечные очки",
        "Громко плачу",
        "След от поцелуя",
        "Два сердца",
        "Сердце",
        "Червы",
        "Класс",
        "Пожимаю плечами",
        "Огонь",
        "Переработка",
    ];

    let emojixpress = [
        2.26, 19.1, 25.6, 233.0, 15.2, 22.7, 64.6, 87.5, 6.81, 6.0, 4.72, 24.7, 21.7, 10.0, 118.0,
        3.31, 23.1, 1.74, 4.5, 0.0333,
    ];

    let instagram = [
        1.02, 1.69, 0.774, 7.31, 2.36, 4.26, 11.2, 5.13, 0.636, 0.236, 3.93, 1.35, 2.87, 5.69,
        26.0, 1.82, 3.75, 0.11, 2.49, 0.056,
    ];

    let twitter = [
        87.3, 150.0, 0.0, 2270.0, 264.0, 565.0, 834.0, 432.0, 0.0, 478.0, 198.0, 654.0, 98.7,
        445.0, 1080.0, 697.0, 227.0, 0.0, 150.0, 932.0,
    ];

    let emoji_rows: Vec<(String, f64, f64, f64)> = names_ru
        .iter()
        .zip(emojixpress.iter())
        .zip(instagram.iter())
        .zip(twitter.iter())
        .map(|(((name, &emojixpress), &instagram), &twitter)| {
            (name.to_string(), emojixpress, instagram, twitter)
        })
        .collect();
    println!("{:?}", emoji_rows);
    println!("\n");

    // Задание 1.
    // Создайте список с элементами первой строки таблицы и распечатайте его на экране.
    let first_row: Vec<_> = emoji_rows.iter().map(|row| row.0.to_string()).collect();
    println!("Задание 1.");
    println!("Первая строка таблицы: {:?}", first_row);
    println!("\n");

    // Задание 2.
    // Найдите сумму первых пяти элементов подготовленного списка emojixpress.
    let first_five_sum_emojixpress: f64 = emojixpress.iter().take(5).sum();
    println!("Задание 2.");
    println!(
        "Сумма первых пяти элементов списка emojixpress: {:.2}",
        first_five_sum_emojixpress
    );
    println!("\n");

    // Задание 3.
    // Всего в сообщениях с клавиатурой EmojiXpress отправлено 1.72 миллиарда, или
    // 1720 миллионов, эмодзи (источник EmojiStats, данные на конец 2018). Для каждого эмодзи из первых десяти посчитайте их долю среди всех. Затем выведите её в процентах с точностью до одного
    // знака после запятой, в следующем формате:
    // Доли эмодзи:
    // 0.1%
    // 1.1%
    // ...
    let shares: Vec<f64> = emojixpress
        .iter()
        .take(10)
        .map(|x| x / TOTAL_EMOJI_IN_EMOJIXPRESS * 100.0)
        .collect();
    println!("Задание 3.");
    println!("Доли эмодзи:");
    for share in shares {
        println!("{:.1}%", share);
    }

    println!();
    println!(
        "Всего отправлено эмодзи в emojixpress: {:.2} млн.",
        TOTAL_EMOJI_IN_EMOJIXPRESS
    );
    println!("\n");

    // Задание 4.
    // Посчитайте суммарное количество первых пяти эмодзи оператором присваивания
    // со сложением.
    let mut first_five_sum_emojixpress_assign = 0.0;
    for &value in emojixpress.iter().take(5) {
        first_five_sum_emojixpress_assign += value;
    }
    println!("Задание 4.");
    println!(
        "Сумма первых пяти элементов списка emojixpress (оператор присваивания): {:.2}",
        first_five_sum_emojixpress_assign
    );
    println!("\n");

    // Задание 5.
    // Оценим, какую долю в EmojiXpress составляют выбранные нами эмодзи.
    // Всего в сообщениях с клавиатуры EmojiXpress их отправлено 1.72 миллиарда, или 1720 миллионов.
    // Сложите количества всех эмодзи из таблицы и сумму поделите на 1720.
    // Результат выведите в процентах с точностью до одного знака после запятой (уже сделано в прекоде).
    let sum_emojixpress: f64 = emojixpress.iter().sum();
    let selected_emojis_share_emojixpress = sum_emojixpress / TOTAL_EMOJI_IN_EMOJIXPRESS * 100.0;
    println!("Задание 5.");
    println!(
        "Доля выбранных эмодзи в EmojiXpress: {:.1}%",
        selected_emojis_share_emojixpress
    );

    let sum_twitter: f64 = twitter.iter().sum();
    let selected_emojis_share_twitter = sum_twitter / TOTAL_EMOJI_IN_TWITTER * 100.0;
    println!(
        "Доля выбранных эмодзи в Twitter: {:.1}%",
        selected_emojis_share_twitter
    );
    println!("\n");

    // Задание 6.
    // Добавьте в таблицу еще две строки: для эмодзи «Слёзы радости» и «Подмигиваю
    let first_five_emoji_rows: Vec<(String, f64, f64, f64)> = names_ru
        .iter()
        .zip(emojixpress.iter())
        .zip(instagram.iter())
        .zip(twitter.iter())
        .take(5)
        .map(|(((name, &emojixpress), &instagram), &twitter)| {
            (name.to_string(), emojixpress, instagram, twitter)
        })
        .collect();
    println!("Задание 6.");
    println!("Первые пять строк таблицы: {:?}", first_five_emoji_rows);
    println!("\n");

    // Задание 7.
    // Допишите код, чтобы он распечатывал всю таблицу в таком формате:
    // Название эмодзи | EmojiXpress, млн | Instagram, млн | Твиттер, млн
    // ------------------------------------------------------------------
    // Ухмыляюсь | 2.26 | 1.02 | 87.3
    // ...
    println!("Задание 7.");
    println!(" Название эмодзи  |  EmojiXpress, млн  |  Instagram, млн  |  Твиттер, млн ");
    println!("--------------------------------------------------------------------------");

    emoji_rows.iter().for_each(|row| {
        println!(
            "{:<16}  |  {:<16}  |  {:<14}  |  {:<10}",
            row.0,
            format!("{:.2}", row.1),
            format!("{:.2}", row.2),
            format!("{:.2}", row.3)
        );
    });
    println!("\n");

    // Задание 8.
    // Допишите код, чтобы он выводил текст:
    // • в ячейке шириной в 15 символов;
    // • с выравниванием по правому краю;
    // • с заполнением пустот пробелами
    println!("Задание 8.");
    println!("|{:>15}|{:>15}|{:>15}|{:>15}|",
             "Название", "EmojiXpress", "Instagram", "Твиттер");
    println!("-----------------------------------------------------------------");
    emoji_rows.iter().for_each(|row| {
        println!(
            "|{:>15}|{:>15.2}|{:>15.2}|{:>15.2}|",
            row.0, row.1, row.2, row.3
        );
    });
    println!("\n");

    // Задание 9.
    // Допишите код, чтобы он выводил число:
    // • в ячейке шириной в 12 символов;
    // • с выравниванием по левому краю;
    // • с заполнением пустот пробелами;
    // • с точностью до двух знаков после запятой.
    println!("Задание 9.");
    println!(
        "|{:<12}|{:<12}|{:<12}|{:<12}|",
        "Название", "EmojiXpress", "Instagram", "Твиттер"
    );
    println!("---------------------------------------------------------------");
    emoji_rows.iter().for_each(|row| {
        println!(
            "|{:<12}|{:<12.2}|{:<12.2}|{:<12.2}|",
            row.0, row.1, row.2, row.3
        );
    });
    println!("\n");

    // Задание 10.
    println!("Задание 10.");
    println!("Название эмодзи  | EmojiXpress, млн | Instagram, млн | Твиттер, млн");
    println!("-------------------------------------------------------------------");
    emoji_rows.iter().for_each(|row| {
        println!(
            "{:<16} | {:<16.2} | {:<14.2} | {:<12.2}",
            row.0, row.1, row.2, row.3,
        );
    });
    println!("\n");

    // Задание 11-12.
    // Для всех топовых эмодзи посчитайте,
    // сколько в среднем (в миллионах) сообщений с каждым из них отправляется в EmojiXpress, Instagram и Twitter.
    println!("Задание 11-12.");
    let avg_emjxpress: f64 = emojixpress.into_iter().avg().unwrap();
    let avg_instagram: f64 = instagram.into_iter().avg().unwrap();
    let avg_twitter: f64 = twitter.into_iter().avg().unwrap();

    println!(
        "Среднее количество сообщений в EmojiXpress: {:.2} млн.",
        avg_emjxpress
    );
    println!(
        "Среднее количество сообщений в Instagram: {:.2} млн.",
        avg_instagram
    );
    println!(
        "Среднее количество сообщений в Twitter: {:.2} млн.",
        avg_twitter
    );
    println!("\n");

    // Задание 13.
    // Посчитайте для каждого эмодзи соотношение его количества в Твиттере к количеству в Instagram.
    // Выведите результат на экран в формате как в примере ниже.
    // Все соотношения выводите с точностью до двух знаков после запятой.
    println!("Задание 13.");
    println!(
        "{: <16} | {: >29}",
        "Название эмодзи", "Соотношение Твиттер/Instagram"
    );
    println!("------------------------------------------------");
    emoji_rows.iter().for_each(|(name, _, instagram, twitter)| {
        let twitter_instagram_ratio = if *instagram != 0.0 {
            twitter / instagram
        } else {
            0.0
        };
        println!("{: <16} | {: >29.2}", name, twitter_instagram_ratio);
    });
    println!("\n");

    // Задание 14.
    // Названия столбцов, образующие «шапку» таблицы, хранятся в списке header.
    // Выведите «шапку» на экран в таком виде:
    // | Название | EmojiXpress, млн | Instagram, млн | Твиттер, млн |
    println!("Задание 14.");
    let header = [
        "Название",
        "EmojiXpress, млн",
        "Instagram, млн",
        "Твиттер, млн",
    ];
    let formatted_header: String = header
        .iter()
        .enumerate()
        .map(|(i, col)| {
            if i == header.len() - 1 {
                format!(" {}", col) // Для последнего элемента без "|"
            } else {
                format!(" {} |", col) // Для всех остальных добавляем "|"
            }
        })
        .collect();
    println!("|{}", formatted_header);
    println!("\n");

    // Задание 15.
    // Расставьте названия всех эмодзи по алфавиту и выведите список на экран, печатая каждый элемент с новой строки.
    println!("Задание 15.");
    names_ru.iter().sorted().for_each(|name| {
        println!("{}", name);
    });
    println!("\n");

    // Задание 16.
    // Отсортируйте таблицу по столбцу «Instagram, млн» по убыванию и выведите её
    // в наглядном формате (см. прекод).
    // Обратите внимание, какие эмодзи наиболее популярны на этой платформе.
    println!("Задание 16.");
    let sorted_by_instagram_reverse: Vec<_> = sort_by_column(&emoji_rows, |row| row.2);
    println!("Название эмодзи  | EmojiXpress, млн | Instagram, млн | Твиттер, млн");
    println!("-------------------------------------------------------------------");
    sorted_by_instagram_reverse
        .iter()
        .for_each(|(name, emojixpress, instagram, twitter)| {
            println!(
                "{: <16} | {: >16.2} | {: >14.2} | {: >12.2}",
                name, emojixpress, instagram, twitter
            );
        });
    println!("\n");

    // Задание 17.
    // Отсортируйте таблицу по столбцу «Твиттер, млн» по убыванию и выведите её в
    // наглядном формате (см. прекод).
    // Обратите внимание, какие эмодзи наиболее популярны на этой платформе.
    // Какой артефакт (необычный объект) выделяется особо?
    println!("Задание 17.");
    let sorted_by_twitter_reverse: Vec<_> = sort_by_column(&emoji_rows, |row| row.3);
    println!("Название эмодзи  | EmojiXpress, млн | Instagram, млн | Твиттер, млн");
    println!("-------------------------------------------------------------------");
    sorted_by_twitter_reverse
        .iter()
        .for_each(|(name, emojixpress, instagram, twitter)| {
            println!(
                "{: <16} | {: >16.2} | {: >14.2} | {: >12.2}",
                name, emojixpress, instagram, twitter
            );
        });
    println!("\n");

    // Задание 18.
    // Сделайте срез, чтобы получить из списка следующие значения:
    // «Глаза-сердца», «Целую», «Задумчивость».
    println!("Задание 18.");
    let selected_emojis = &names_ru[6..9];
    println!(
        "Срез из 'Глаза-сердца', 'Целую', 'Задумчивость': {:?}",
        selected_emojis
    );
    println!("\n");

    // Задание 19.
    // Обновите код, чтобы в каждом из трёх случаев выводилась не вся таблица, а только её первые пять строк.
    println!("Задание 19.");
    let sorted_by_emojixpress_reverse: Vec<_> = sort_by_column(&emoji_rows, |row| row.1);
    println!("Название эмодзи  | EmojiXpress, млн");
    println!("-----------------------------------");
    sorted_by_emojixpress_reverse
        .iter()
        .take(5)
        .for_each(|(name, emojixpress, _, _)| {
            println!("{: <16} | {: >16.2}", name, emojixpress);
        });
    println!("\n");

    println!("Название эмодзи  | Instagram, млн");
    println!("---------------------------------");
    sorted_by_instagram_reverse
        .iter()
        .take(5)
        .for_each(|(name, _, instagram, _)| {
            println!("{: <16} | {: >14.2}", name, instagram);
        });
    println!("\n");

    println!("Название эмодзи  | Твиттер, млн");
    println!("-------------------------------");
    sorted_by_twitter_reverse
        .iter()
        .take(5)
        .for_each(|(name, _, _, twitter)| {
            println!("{: <16} | {: >12.2}", name, twitter);
        });
    println!("\n");

    // Задание 20.
    // Измените значения списка twitter: прибавьте к каждому элементу 0.001.
    // Напечатайте на экране результат.
    println!("Задание 20.");
    let twitter_updated: Vec<_> = twitter.iter().map(|x| x + 0.001).collect();
    println!("Измененный Twitter: {:?}", twitter_updated);
    println!("\n");

    // Задание 21.
    // Добавьте в список названий emoji ещё три следующих элемента.
    // Выведите результат на экран.
    println!("Задание 21.");
    let five_emoji = &names_ru[0..5];
    let eight_emoji: Vec<&str> = five_emoji
        .iter()
        .chain(["Солнечные очки", "Громко плачу", "След от поцелуя"].iter())
        .cloned()
        .collect();
    println!("Восемь эмоджи: {:?}", eight_emoji);
    println!("\n");

    // Задание 22.
    // Создайте в таблице новый столбец: суммарное количество использований на всех платформах.
    // Отсортируйте таблицу по нему по убыванию и напечатайте на экране первые пять элементов.
    println!("Задание 22.");
    let emoji_rows_with_total: Vec<(String, f64, f64, f64, f64)> = emoji_rows
        .iter()
        .map(|(name, emojixpress, instagram, twitter)| {
            (
                name.to_string(),
                *emojixpress,
                *instagram,
                *twitter,
                emojixpress + instagram + twitter,
            )
        })
        .collect();

    let sorted_by_total_reverse: Vec<_> = sort_by_column(&emoji_rows_with_total, |row| row.4);
    println!("Название эмодзи  | EmojiXpress, млн | Instagram, млн | Твиттер, млн | Сумма");
    println!("---------------------------------------------------------------------------");
    sorted_by_total_reverse.iter().take(5).for_each(
        |(name, emojixpress, instagram, twitter, total)| {
            println!(
                "{: <16} | {: >16.2} | {: >14.2} | {: >12.2} | {: >5.2}",
                name, emojixpress, instagram, twitter, total
            );
        },
    );
    println!("\n");

    // Задание 23.
    // Смотри задание 11-12.

    // Задание 24.
    // Вычислите «индекс использования» для каждого эмодзи.
    // Получившийся столбец с этими индексами добавьте к таблице.
    println!("Задание 24.");
    let emoji_rows_with_usage_index: Vec<(String, f64, f64, f64, f64)> = emoji_rows
        .iter()
        .map(|(name, emojixpress, instagram, twitter)| {
            let normalized_emojixpress = *emojixpress / avg_emjxpress;
            let normalized_instagram = *instagram / avg_instagram;
            let normalized_twitter = *twitter / avg_twitter;
            let usage_index = normalized_emojixpress + normalized_instagram + normalized_twitter;
            (
                name.to_string(),
                *emojixpress,
                *instagram,
                *twitter,
                usage_index,
            )
        })
        .collect();
    println!("Название эмодзи  | Индекс использования");
    println!("---------------------------------------");
    emoji_rows_with_usage_index
        .iter()
        .for_each(|(name, _, _, _, usage_index)| {
            println!("{: <16} | {: >20.2}", name, usage_index);
        });
    println!("\n");

    // Задание 25.
    // Финальный шаг для получения отчёта:
    // отсортируйте таблицу по новому столбцу и напечатайте на экране только первые пять элементов.
    println!("Задание 25.");
    let sorted_by_emojixpress: Vec<_> = sort_by_column(&emoji_rows_with_usage_index, |row| row.1);
    println!("Название эмодзи  | Emojixpress, млн");
    println!("-----------------------------------");
    sorted_by_emojixpress
        .iter()
        .take(5)
        .for_each(|(name, emojixpress, _, _, _)| {
            println!("{: <16} | {: >14.2} |", name, emojixpress);
        });
    println!("\n");

    let sorted_by_instagram: Vec<_> = sort_by_column(&emoji_rows_with_usage_index, |row| row.2);
    println!("Название эмодзи  | Instagram, млн");
    println!("---------------------------------");
    sorted_by_instagram
        .iter()
        .take(5)
        .for_each(|(name, _, instagram, _, _)| {
            println!("{: <16} | {: >12.2} |", name, instagram);
        });
    println!("\n");

    let sorted_by_twitter: Vec<_> = sort_by_column(&emoji_rows_with_usage_index, |row| row.3);
    println!("Название эмодзи  | Twitter, млн");
    println!("-------------------------------");
    sorted_by_twitter
        .iter()
        .take(5)
        .for_each(|(name, _, _, twitter, _)| {
            println!("{: <16} | {: >10.2} |", name, twitter);
        });
    println!("\n");

    let sorted_by_usage_index = sort_by_column(&emoji_rows_with_usage_index, |row| row.4);
    println!("Название эмодзи  | Индекс использования");
    println!("---------------------------------------");
    sorted_by_usage_index
        .iter()
        .take(5)
        .for_each(|(name, _, _, _, usage_index)| {
            println!("{: <16} | {: >18.2} |", name, usage_index);
        });
    println!("\n");
    Ok(())
}
