/*use crate::parser::parser_new::CommentBlock;

fn generate_document(comments: Vec<CommentBlock>, output_format: &str) {
    for comment in comments {
        match output_format {
            "html" => println!("{}", comment.generate_html()),
            "markdown" => println!("{}", comment.generate_markdown()),
            _ => eprintln!("Unsupported format"),
        }
    }
}

fn main() {
    let comment = r#"
    /**
     * @Type Function
     * @Brief これはサンプルの関数です
     * @Detail この関数はドキュメント生成ツールの例として作られました
     * @Param x 引数xは整数
     * @Return 結果の整数値を返します
     */
    "#;

    let parsed_comment = parse_comment_block(comment);
    generate_document(vec![parsed_comment], "html");
}
*/