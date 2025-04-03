use std::io::{self, Write};
use vader_sentiment::SentimentIntensityAnalyzer;

fn main() {
    let analyzer = SentimentIntensityAnalyzer::new();

    loop {
        print!("Enter text to analyze sentiment (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let sentiment = analyzer.polarity_scores(input);

        let compound = sentiment.get("compound").unwrap_or(&0.0);
        let pos = sentiment.get("pos").unwrap_or(&0.0);
        let neu = sentiment.get("neu").unwrap_or(&0.0);
        let neg = sentiment.get("neg").unwrap_or(&0.0);

        println!("Compound: {}", compound);
        println!("Positive: {}", pos);
        println!("Neutral: {}", neu);
        println!("Negative: {}", neg);
    }
}