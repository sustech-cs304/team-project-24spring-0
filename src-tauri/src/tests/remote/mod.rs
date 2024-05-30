mod helper;
mod server_test;

use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::types::middleware_types::TabMap;

static TEST_FILE_NAME: &str = "/foo/bar/test_file.txt";

static TEST_PASSWD: &str = "wasd";

static MAX_PORT_RETRY: usize = 1145;

static TAB_MAP: Lazy<Mutex<Option<TabMap>>> = Lazy::new(|| Mutex::new(None));

static CURSOR_LIST: Lazy<Mutex<Option<crate::rpc_types::CursorListState>>> =
    Lazy::new(|| Mutex::new(None));

static TEST_FILE_CONTENT: &str = r#"
你说的对，但是《软件工程》是由SUSTech自主研发的一款经典小组合作冒险游戏。
游戏发生在一个被曾被叫做「SUSTC」的申比世界，在这里，随机组队的你将被授予「祭拜Project」，获得救火之力。
你将扮演一位名为「牢大」的神秘角色，在吃食的开发坐牢中认识到队友们的真面目，和他们一起列出 User Stories，完成狗屎项目。
同时，逐步发掘「去行政化」和「教学改革」的真相。

Man, what can I say?

....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................~=*^^^^^^^^^^*****=*^^^^*.......................................................
...................................................................-=*^>>>><<<(<<<<<>>>>>>^^=~.........-~~~~>^......................................................
...................................-~=~=*^^*=..................-*^*=.....^^^>^..............=^^^^^^^=..~~~~~~^^.....................................................
..............................-><((<(<^=....~><<>~............*^~~~~..=^*....^=........~*^^*-*^......-^>>^~~~~^^....................................................
............................>(((*...............~<<>.........^>~~~~-*^=......-^=....*^^=.....=^...........><>==**...................................................
..........................<((-.....................^<>......*>~~~~*^=..........^*-^^~........~^..............><>^~..................................................
........................=((.......................--->>-....>*~~=^*.............*^-..........-^-.....~**~**.....^(<.................................................
.......................*(>........*>>>>>>>>>^-...-~---><=..*>~~^^~.=*^^>><<>>>>>>>>>>>>>>^*-..^=.-**=....*^...-<<~^<>=..............................................
......................~<>.......^>>^.......->>>.-------<<~.>^=>>^^=~~~~~~~~~~~~=*~~~~~~~~~~~^><(<=.......*^..><~--~>*^<^............................................
......................*<=......>^^...........^><---~---*(>.^><>~~~~~~~~~~~~----^=-----~~~~~~~~~~~~^^^=....^><>=---~<>==*^^..........................................
......................><......*>^.............<<<---~-~-<<-><---..------------=^-----------.---~~~~~~~=*=-><^^~----<<==*^^<>-.......................................
......................<<-.....^>^............-^<<-------<(<>..-.-....-------.-<=-----------.-.-..--~~~~~><>^^^~~---<<^*==^<((<......................................
......................<(=.....=>^..........---><<-~-~-~-((^-.------------.--=>>-.---...-----------.---~<>^^^*^*---~<<=><(*=*>(>=....................................
......................>(^......^^>.......----><<=-~---~*(<-.-.-..---.----.-^^>>.-....--..-.---------.->>*^^^^^^=--~((<>-----^<*<>...................................
......................-((.......*>^*...----><<<----~---((-....-.-=~..----=>=->^-.........-.--------~-*<^^^^^^*^^*~^^^>=-~-~=<<==<^..................................
.......................*<<........>><<<<<<<<<--------*((-....-.-^=---.--^>---<=.-.....-.-------..-*--<>^^^*^*^^^<(((<=~~~^^^(>*=<^..................................
........................*(<......-----~~~-----~--~--<((~.---..-^^------>*---~<~..-.--.-.-------.~>~-=<^^*^^^^><<^^(<^^^^^*^><*>>....................................
..........................>(>~-------~----~-~---=<<(<>>-.----..>=--.-*>~----~<~-.---------------><*-=<^^^^^((>^^*><<<*^^*^^<<^-.....................................
............................*<<<=-----~----~--=<<=...<~--.----->=-.-^>-------<=--------------.->^*<=-^><<<<^^^*^>^<^<^^^^^((........................................
...............................=><<<<<<>>^^~---<<...^>---------<>.-<(-------->>~^=-.-------.-->>--*>~.*<^^*^*^^<^=<^><^*^((>........................................
..........................................^>--~~<^..>>----.-..-(<-^>(((>---..^<~>**~--..-.---<>----*>--~<<*^^*<>-*<^^<<^<<<(........................................
...........................................>>---<<..>>-----^---<><^...<(((-...<*<-=>~~~~~~--<<----..=<*=-=<<>>*-~=<^*^<<^*>(~.......................................
...........................................=>*--=<^.^<~~~~^<=~~<^^......=((<..=>>~.->*~~~~~<(--<((((((((*.^>---->=<^^^>><<*<^.......................................
............................................><---<<--<~~~~^~<~~^>......~<((((~.~*....^>~~~((((((^......><->*.--->^~***=~-.-<>.......................................
.............................................>>---<>.>^~~=>~><~<>-<(((((<^-............^<(<^(((<........-<(.-..->>~~~~~~~-.>>.......................................
.............................................*<^--=<*.>*~*>.^~(<..~-..........................>(((((=....>^--..-<^~~~~~~~-.~<~......................................
..............................................<(~-->>-.^^~>.*^~<>=................................-(((^.>>.-.-.-<*~~~~~~~~--<^......................................
...............................................<(><<<><<<<(<<(>>*=><>................................-><<----.-~<=~~~~~~~~-.*>......................................
...............................................^<-...-(=.-**..=<<*<<*^><^^^*~.....................^><(<(~.-----><~~~~~~~~~~-.>^.....................................
...............................................-<=....(<..**...-<*=>((*=><*~*<]((<<<<<<<>>><<(<((((<^><~-.---.-<>~~~~~~~~~~--*>.....................................
..................................................>((<((-.*^=~~~^<.=((==*<<~=^*^>((((((((>>><(>^====<<~~~----~*<~~~~~~~~~~~~-->^....................................
...................................................>(*~<<~~>^~~~~(<*>*.==<(^^^^^*^<>>^^=~~^^<<====^<>~~~*^~~~=<^~~~~~~~~~~~~~-->-...................................
....................................................<<<<<*~>^~=*<(<~*^>^=^<(<(<<<>((<^^^^^^><>===<<><>^-.^^=<<~~~~~~~~~~~~~~~--~>...................................
........................................................><<<<<><>~><~..^^..-.....===->><<<<<*..~*^>^^~...^^~.><>~~~~~~~~~~~~~~-.^^..................................
..............................................................-^^~=<<~.*>~......-=*==.....~><<<>^^*~~^><<=....=<<~~~~~~~~~~~~~~-->*.--~--...........................
...............................................-~~=***^>>>^^>>>>>><<<(<<<(((<<<<<<<(<<<<<<<><<<<>*.=*=..<(((<<(<(<<<<<<<<<<<<<<<<<<^=--.............................
.........................................................................................^^......^<<<(((>~~~~~~-....................................................
..........................................................................................^>>>>>>>>*~..-............................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................
....................................................................................................................................................................

程序和人有一个能跑就行
"#;
