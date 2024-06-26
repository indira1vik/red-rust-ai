#![windows_subsystem = "windows"]

use rust_bert::pipelines::sentiment::SentimentModel;
use rust_bert::pipelines::sentiment::SentimentPolarity;
use rust_bert::pipelines::summarization::SummarizationModel;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    let main_window_handle: slint::Weak<MainWindow> = main_window.as_weak();

    let summarizer_handle = main_window_handle.clone();
    main_window.on_receive_search_content(move |query| {
        bert_summarizer(summarizer_handle.clone(), query.to_string())
    });

    let chatbot_handle = main_window_handle.clone();
    main_window
        .on_receive_bot_content(move |query| analysis(chatbot_handle.clone(), query.to_string()));

    main_window.run()
}

fn bert_summarizer(window_handle: slint::Weak<MainWindow>, query: String) {
    let window_handle = window_handle.unwrap();

    let model =
        SummarizationModel::new(Default::default()).expect("Failed to create SummarizationModel");
    if query == "" {
        let error_msg = "Result => Empty Query... Please enter search content...\n";
        let user_input = "Query => ".to_string() + &query + "\n\n";
        let reply_value = user_input + &error_msg;
        window_handle.set_answer(window_handle.get_answer() + &reply_value);
    } else {
        let user_input = "Query => ".to_string() + &query + "\n\n";
        let input = [query.as_str()];
        let output: Vec<String> = model.summarize(&input);
        let final_output = output.join("\n");
        let result_intro = "Result => ".to_string();
        let reply_value = user_input + &result_intro + &final_output;
        window_handle.set_answer(window_handle.get_answer() + &reply_value + "\n\n");
    }
}

fn analysis(window_handle: slint::Weak<MainWindow>, query: String) {
    let window_handle = window_handle.unwrap();

    if query == "" {
        let error_msg = "Result => Empty Query... Please enter search content...\n\n".to_string();
        let user_input = "Query => ".to_string() + &query + "\n\n";
        let reply_value = user_input + &error_msg;
        window_handle.set_reply(window_handle.get_reply() + &reply_value);
    } else {
        let sentiment_model: SentimentModel =
            SentimentModel::new(Default::default()).expect("Failed to load Sentimental Model");
        let user_input = "Query => ".to_string() + &query + "\n\n";
        let input = [query.as_str()];
        let output = sentiment_model.predict(&input);
        let joined_values: String = output
            .iter()
            .map(|sentiment| match sentiment.polarity {
                SentimentPolarity::Positive => "Positive".to_string(),
                SentimentPolarity::Negative => "Negative".to_string(),
            })
            .collect::<Vec<String>>()
            .join("\n");
        let result_intro = "Result => ".to_string();
        let reply_value = user_input + &result_intro + &joined_values;
        window_handle.set_reply(window_handle.get_reply() + &reply_value + "\n\n");
    }
}
