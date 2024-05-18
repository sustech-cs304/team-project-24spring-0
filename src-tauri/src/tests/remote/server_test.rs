static TEST_FILE_CONTENT: &str = r#"
你说的对，但是《软件工程》是由SUSTech自主研发的一款经典小组合作冒险游戏。
游戏发生在一个被曾被叫做「SUSTC」的申比世界，在这里，随机组队的你将被授予「祭拜Project」，获得救火之力。
你将扮演一位名为「牢大」的神秘角色，在吃食的开发坐牢中认识到队友们不回消息、能力清奇的一面，和他们一起满足 User Stories，完成狗屎项目
——
同时，逐步发掘「木琴去哪」的真相。
- “工程量有点大”
- “能跑就行”
- “明星公司”
- “把需求给我列个单子”
"#;

#[test]
fn test_authorize() {
    let mut server = init_test_server(TEST_FILE_CONTENT).unwrap();
    assert!(1 == 1, "foo");
}
