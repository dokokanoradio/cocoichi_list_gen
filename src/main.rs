use itertools::Itertools;
use std::fs;
use std::io::{BufWriter, Write};
use std::time;

fn main() {
    // 改行文字
    #[cfg(windows)]
    let line_ending = "\r\n";
    #[cfg(not(windows))]
    let line_ending = "\n";

    // トッピング
    let topping = [
        "手仕込チキンカツ", //期間限定
        "ハーフチキンにこみ",
        "ミニももカツ",
        "ハーフ豚しゃぶ",
        "ハンバーグ1個",
        "フライドチキン3個",
        "ソーセージ（2本）",
        "手仕込とんかつ",
        "ソーセージ（4本）",
        "メンチカツ",
        "豚しゃぶ",
        "パリパリチキン",
        "チキンカツ",
        "ロースカツ",
        "フライドチキン5個",
        "チキンにこみ",
        "2個カキフライ", //期間限定
        "カキフライ",    //期間限定
        "フィッシュフライ1本",
        "ハーフあさり",
        "ハーフエビにこみ",
        "ハーフイカ",
        "エビフライ",
        "エビカツ",
        "海の幸",
        "エビにこみ",
        "イカ",
        "たっぷりあさり",
        "ツナ",
        "大根と生姜のピクルス", //期間限定
        "ハーフやさい",
        "ハーフなす",
        "ハーフほうれん草",
        "なす",
        "やさい",
        "ほうれん草",
        "トマトガーリック",
        "旨辛にんにく",
        "完熟カットトマト",
        "単品ポテト",
        "コーン",
        "ガーリック",
        "うずら卵串フライ",
        "ハーフスクランブルエッグ",
        "クリームコロッケ（カニ入り）1個",
        "ハーフきのこ",
        "ハーフチーズ",
        "チーズ",
        "クリームコロッケ（カニ入り）",
        "きのこ",
        "スクランブルエッグ",
        "納豆",
        "半熟タマゴタルタルソース",
        "ゆでタマゴ",
        "半熟タマゴ",
        "クリーミータルタルソース",
    ];

    // 辛さの度合い
    let spiciness = [
        "普通", "1辛", "2辛", "3辛", "4辛", "5辛", "6辛", "7辛", "8辛", "9辛", "10辛", "15辛",
        "20辛",
    ];

    // 甘さの度合い
    let sweetness = [
        "", //0甘
        "1甘", "2甘", "3甘", "4甘", "5甘",
    ];

    // ルー
    let roux = [
        "ポーク",
        "甘口ポーク",
        "ビーフカレー",
        "ココイチベジ",
        "ハヤシ", //カレーではないがココイチのメニューなので
    ];

    //(辛さ/甘さを加えた)ルーの一覧
    let mut roux_vec: Vec<String> = vec![];

    //一覧の作成
    for r in roux {
        for sp in spiciness {
            for sw in sweetness {
                //ポーク2辛3甘 の感じで文字列作る
                roux_vec.push(format!("{}{}{}", r, sp, sw));
            }
        }
    }

    //結果出力用のファイルを作る
    let mut f = BufWriter::new(fs::File::create("cocoichi.txt").unwrap());

    // 個数
    let mut count: u64 = 0;

    // 今の時刻を取得
    let now = time::Instant::now();

    // トッピング0個
    println!("トッピング0個を出力");
    for r in &roux_vec {
        f.write(format!("{}{}", r, line_ending).as_bytes()).unwrap();
        count += 1;
    }
    println!("ここまでの所要時間：{:?}", now.elapsed());

    // トッピング1個
    println!("トッピング1個を出力");
    for tp in topping.iter().combinations(1) {
        let tp_text = format!("({})", tp[0]); // トッピング文字列
        for r in &roux_vec {
            f.write(format!("{}{}{}", r, tp_text, line_ending).as_bytes())
                .unwrap();
            count += 1;
        }
    }
    println!("ここまでの所要時間：{:?}", now.elapsed());

    // トッピング2個
    println!("トッピング2個を出力");
    for tp in topping.iter().combinations(2) {
        let tp_text = format!("({}/{})", tp[0], tp[1]); // トッピング文字列
        for r in &roux_vec {
            f.write(format!("{}{}{}", r, tp_text, line_ending).as_bytes())
                .unwrap();
            count += 1;
        }
    }
    println!("ここまでの所要時間：{:?}", now.elapsed());

    // トッピング3個
    println!("トッピング3個を出力");
    for tp in topping.iter().combinations(3) {
        let tp_text = format!("({}/{}/{})", tp[0], tp[1], tp[2]); // トッピング文字列
        for r in &roux_vec {
            f.write(format!("{}{}{}", r, tp_text, line_ending).as_bytes())
                .unwrap();
            count += 1;
        }
    }
    println!("ここまでの所要時間：{:?}", now.elapsed());

    // トッピング4個
    println!("トッピング4個を出力");
    for tp in topping.iter().combinations(4) {
        let tp_text = format!("({}/{}/{}/{})", tp[0], tp[1], tp[2], tp[3]); // トッピング文字列
        for r in &roux_vec {
            f.write(format!("{}{}{}", r, tp_text, line_ending).as_bytes())
                .unwrap();
            count += 1;
        }
    }
    println!("ここまでの所要時間：{:?}", now.elapsed());

    // トッピング5個
    println!("トッピング5個を出力");
    for tp in topping.iter().combinations(5) {
        let tp_text = format!("({}/{}/{}/{}/{})", tp[0], tp[1], tp[2], tp[3], tp[4]); // トッピング文字列
        for r in &roux_vec {
            f.write(format!("{}{}{}", r, tp_text, line_ending).as_bytes())
                .unwrap();
            count += 1;
        }
    }
    println!("ここまでの所要時間：{:?}", now.elapsed());
    println!("{}", format!("組み合わせ数：{}組", count));
}
