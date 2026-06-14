use rand::seq::SliceRandom;

pub fn phrase(category: &str) -> &'static str {
    let mut rng = rand::thread_rng();

    match category {
        "startup" => STARTUP.choose(&mut rng).unwrap(),
        "thinking" => THINKING.choose(&mut rng).unwrap(),
        "scanning" => SCANNING.choose(&mut rng).unwrap(),
        "saving" => SAVING.choose(&mut rng).unwrap(),
        "exporting" => EXPORTING.choose(&mut rng).unwrap(),
        "connecting" => CONNECTING.choose(&mut rng).unwrap(),
        "performing" => PERFORMING.choose(&mut rng).unwrap(),
        "tool_call" => TOOL_CALL.choose(&mut rng).unwrap(),
        "success" => SUCCESS.choose(&mut rng).unwrap(),
        "error" => ERROR.choose(&mut rng).unwrap(),
        "blank" => BLANK.choose(&mut rng).unwrap(),
        "token" => TOKEN.choose(&mut rng).unwrap(),
        "council" => COUNCIL.choose(&mut rng).unwrap(),
        _ => STARTUP.choose(&mut rng).unwrap(),
    }
}

const STARTUP: &[&str] = &[
    "tiny paws, serious patches",
    "pawing through code, softly",
    "coding claws for every terminal",
    "from Termux caves to x86 castles",
    "less tokens, more outcome",
];

const THINKING: &[&str] = &[
    "pawing through the thought yarn...",
    "tiny brain, big repo...",
    "sniffing the bug trail...",
    "calculating with whiskers...",
];

const SCANNING: &[&str] = &[
    "sniffing the project tree...",
    "checking every cozy folder...",
    "looking for code crumbs...",
    "reading the cave walls...",
];

const SAVING: &[&str] = &[
    "saving pawprints...",
    "tucking memory into the basket...",
    "writing this down with tiny claws...",
];

const EXPORTING: &[&str] = &[
    "wrapping the result in a ribbon...",
    "packing the patch bundle...",
    "exporting pawprints...",
];

const CONNECTING: &[&str] = &[
    "calling the model moon...",
    "opening a tiny portal...",
    "connecting whiskers to provider...",
];

const PERFORMING: &[&str] = &[
    "doing the tiny work...",
    "claws on keyboard...",
    "soft paws, sharp moves...",
];

const TOOL_CALL: &[&str] = &[
    "using claws carefully...",
    "borrowing a tool from the pouch...",
    "paw step incoming...",
];

const SUCCESS: &[&str] = &[
    "purrfect, done.",
    "patch landed softly.",
    "the yarn has been untangled.",
];

const ERROR: &[&str] = &[
    "hiss. that did not land.",
    "tiny paws slipped.",
    "something scratched back.",
];

const BLANK: &[&str] = &[
    "nothing here yet, tiny paws waiting.",
    "empty basket, ready for code.",
    "quiet terminal, curious cat.",
];

const TOKEN: &[&str] = &[
    "saving tokens like fish snacks...",
    "small context, big claws...",
    "less fluff, more bite...",
];

const COUNCIL: &[&str] = &[
    "calling the cat council...",
    "tiny agents gathering...",
    "planner and reviewer entering the den...",
];
