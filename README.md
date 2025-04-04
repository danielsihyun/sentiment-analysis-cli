Sentiment Analysis CLI Tool

Overview
This is a simple **Command Line Interface (CLI) tool** for performing sentiment analysis on text using **Rust**. It utilizes the **vader_sentiment** library to classify text into positive, neutral, and negative sentiments. 

Features
- Analyze sentiment for any English text input.
- Outputs four sentiment scores: **Compound, Positive, Neutral, and Negative**.
- Simple CLI interface.
- Written in Rust for performance optimization.

Installation

Prerequisites
- Install Rust & Cargo (if not already installed):
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  
- Install Homebrew (Mac users only, if not installed):
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  
- Install Git (if not installed):
  brew install git

Clone the Repository:
  git clone https://github.com/your-username/sentiment-cli.git
  cd sentiment-cli


Build and Run the Project
  cargo build
  cargo run


Usage
Once the program is running, enter text to analyze its sentiment:
Enter text to analyze sentiment (or type 'exit' to quit): I love Rust programming!

Example output:
Compound: 0.75
Positive: 0.85
Neutral: 0.10
Negative: 0.05

Future Improvements
- Multilingual Support: Translate non-English text before analysis.
- File Input Support for Batch Processing: Analyze large text files instead of just CLI input.
- Web API Integration: Convert this into an HTTP-based API.
