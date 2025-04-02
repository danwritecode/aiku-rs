use leptos::logging;
use regex::Regex;
use once_cell::sync::OnceCell;

static ADD_REGEX: OnceCell<Vec<Regex>> = OnceCell::new();
static SUB_REGEX: OnceCell<Vec<Regex>> = OnceCell::new();
static VALID_REGEX: OnceCell<Regex> = OnceCell::new();


fn init_add_regex() -> Vec<Regex> {
    vec![ 
        Regex::new("cial").unwrap(),
        Regex::new("tia").unwrap(),
        Regex::new("cius").unwrap(),
        Regex::new("cious").unwrap(),
        Regex::new("uiet").unwrap(),
        Regex::new("gious").unwrap(),
        Regex::new("geous").unwrap(),
        Regex::new("priest").unwrap(),
        Regex::new("giu").unwrap(),
        Regex::new("dge").unwrap(),
        Regex::new("ion").unwrap(),
        Regex::new("iou").unwrap(),
        Regex::new("sia$").unwrap(),
        Regex::new(".che$").unwrap(),
        Regex::new(".ched$").unwrap(),
        Regex::new(".abe$").unwrap(),
        Regex::new(".ace$").unwrap(),
        Regex::new(".ade$").unwrap(),
        Regex::new(".age$").unwrap(),
        Regex::new(".aged$").unwrap(),
        Regex::new(".ake$").unwrap(),
        Regex::new(".ale$").unwrap(),
        Regex::new(".aled$").unwrap(),
        Regex::new(".ales$").unwrap(),
        Regex::new(".ane$").unwrap(),
        Regex::new(".ame$").unwrap(),
        Regex::new(".ape$").unwrap(),
        Regex::new(".are$").unwrap(),
        Regex::new(".ase$").unwrap(),
        Regex::new(".ashed$").unwrap(),
        Regex::new(".asque$").unwrap(),
        Regex::new(".ate$").unwrap(),
        Regex::new(".ave$").unwrap(),
        Regex::new(".azed$").unwrap(),
        Regex::new(".awe$").unwrap(),
        Regex::new(".aze$").unwrap(),
        Regex::new(".aped$").unwrap(),
        Regex::new(".athe$").unwrap(),
        Regex::new(".athes$").unwrap(),
        Regex::new(".ece$").unwrap(),
        Regex::new(".ese$").unwrap(),
        Regex::new(".esque$").unwrap(),
        Regex::new(".esques$").unwrap(),
        Regex::new(".eze$").unwrap(),
        Regex::new(".gue$").unwrap(),
        Regex::new(".ibe$").unwrap(),
        Regex::new(".ice$").unwrap(),
        Regex::new(".ide$").unwrap(),
        Regex::new(".ife$").unwrap(),
        Regex::new(".ike$").unwrap(),
        Regex::new(".ile$").unwrap(),
        Regex::new(".ime$").unwrap(),
        Regex::new(".ine$").unwrap(),
        Regex::new(".ipe$").unwrap(),
        Regex::new(".iped$").unwrap(),
        Regex::new(".ire$").unwrap(),
        Regex::new(".ise$").unwrap(),
        Regex::new(".ished$").unwrap(),
        Regex::new(".ite$").unwrap(),
        Regex::new(".ive$").unwrap(),
        Regex::new(".ize$").unwrap(),
        Regex::new(".obe$").unwrap(),
        Regex::new(".ode$").unwrap(),
        Regex::new(".oke$").unwrap(),
        Regex::new(".ole$").unwrap(),
        Regex::new(".ome$").unwrap(),
        Regex::new(".one$").unwrap(),
        Regex::new(".ope$").unwrap(),
        Regex::new(".oque$").unwrap(),
        Regex::new(".ore$").unwrap(),
        Regex::new(".ose$").unwrap(),
        Regex::new(".osque$").unwrap(),
        Regex::new(".osques$").unwrap(),
        Regex::new(".ote$").unwrap(),
        Regex::new(".ove$").unwrap(),
        Regex::new(".pped$").unwrap(),
        Regex::new(".sse$").unwrap(),
        Regex::new(".ssed$").unwrap(),
        Regex::new(".ste$").unwrap(),
        Regex::new(".ube$").unwrap(),
        Regex::new(".uce$").unwrap(),
        Regex::new(".ude$").unwrap(),
        Regex::new(".uge$").unwrap(),
        Regex::new(".uke$").unwrap(),
        Regex::new(".ule$").unwrap(),
        Regex::new(".ules$").unwrap(),
        Regex::new(".uled$").unwrap(),
        Regex::new(".ume$").unwrap(),
        Regex::new(".une$").unwrap(),
        Regex::new(".upe$").unwrap(),
        Regex::new(".ure$").unwrap(),
        Regex::new(".use$").unwrap(),
        Regex::new(".ushed$").unwrap(),
        Regex::new(".ute$").unwrap(),
        Regex::new(".ved$").unwrap(),
        Regex::new(".we$").unwrap(),
        Regex::new(".wes$").unwrap(),
        Regex::new(".wed$").unwrap(),
        Regex::new(".yse$").unwrap(),
        Regex::new(".yze$").unwrap(),
        Regex::new(".rse$").unwrap(),
        Regex::new(".red$").unwrap(),
        Regex::new(".rce$").unwrap(),
        Regex::new(".rde$").unwrap(),
        Regex::new(".ily$").unwrap(),
        Regex::new(".ely$").unwrap(),
        Regex::new(".des$").unwrap(),
        Regex::new(".gged$").unwrap(),
        Regex::new(".kes$").unwrap(),
        Regex::new(".ced$").unwrap(),
        Regex::new(".ked$").unwrap(),
        Regex::new(".med$").unwrap(),
        Regex::new(".mes$").unwrap(),
        Regex::new(".ned$").unwrap(),
        Regex::new(".[sz]ed$").unwrap(),
        Regex::new(".nce$").unwrap(),
        Regex::new(".rles$").unwrap(),
        Regex::new(".nes$").unwrap(),
        Regex::new(".pes$").unwrap(),
        Regex::new(".tes$").unwrap(),
        Regex::new(".res$").unwrap(),
        Regex::new(".ves$").unwrap(),
        Regex::new("ere$").unwrap()
    ]
}


fn init_sub_regex() -> Vec<Regex> {
    vec![
        Regex::new("riet").unwrap(),
        Regex::new("dien").unwrap(),
        Regex::new("ien").unwrap(),
        Regex::new("iet").unwrap(),
        Regex::new("iu").unwrap(),
        Regex::new("iest").unwrap(),
        Regex::new("io").unwrap(),
        Regex::new("ii").unwrap(),
        Regex::new("ily").unwrap(),
        Regex::new(".oala$").unwrap(),
        Regex::new(".iara$").unwrap(),
        Regex::new(".ying$").unwrap(),
        Regex::new(".earest").unwrap(),
        Regex::new(".arer").unwrap(),
        Regex::new(".aress").unwrap(),
        Regex::new(".eate$").unwrap(),
        Regex::new(".eation$").unwrap(),
        Regex::new("[aeiouym]bl$").unwrap(),
        Regex::new("[aeiou]{3}").unwrap(),
        Regex::new("^mc").unwrap(),
        Regex::new("ism").unwrap(),
        Regex::new("^mc").unwrap(),
        Regex::new("asm").unwrap(),
        Regex::new("([^aeiouy])1l$").unwrap(),
        Regex::new("[^l]lien").unwrap(),
        Regex::new("^coa[dglx].").unwrap(),
        Regex::new("[^gq]ua[^auieo]").unwrap(),
        Regex::new("dnt$").unwrap(),
        Regex::new("ia").unwrap()
    ]
}
    
fn init_valid_regex() -> Regex {
    Regex::new(r"[^aeiouy]+").unwrap()
}


pub fn init_syllable_counter() {
    ADD_REGEX.get_or_init(|| init_add_regex());
    SUB_REGEX.get_or_init(|| init_sub_regex());
    VALID_REGEX.get_or_init(|| init_valid_regex());
    logging::log!("Syllable counter initialized");
}

// Estimates the number of syllables in a word. This is a simple heuristic that is not perfect, but should work for most English words.
pub fn estimate_syllables(word: &str) -> usize {
    if word.len() < 1 {
        return 0;
    }

    // Initialise counters
    let mut sub_counter: usize = 0;
    let mut add_counter: usize = 0;

    // Matches will be case-insensitive
    let l_word: &str = &word.to_lowercase()[..];

    // Split and count "valid" syllable part candidates
    let valid_parts: usize = VALID_REGEX.get().unwrap().split(l_word)
        .filter(|x| !x.is_empty())
        .count();

    // Increment counter for regex patterns we need to subtract from our total (patterns that merge syllables)
    sub_counter += SUB_REGEX.get().unwrap().iter()
        .filter(|x| x.captures(l_word).is_some())
        .count();

    // Increment counter for regex matches we need to add to our counter (patterns that create syllables)
    let add_caps: Vec<Option<regex::Captures<'_>>> = ADD_REGEX.get().unwrap().iter()
        .map(|x| x.captures(l_word))
        .filter(|x| x.is_some())
        .collect::<Vec<_>>();

    add_counter += add_caps.len();

    // Check add captures for 
    sub_counter += add_caps.iter()
        .map(
            |x| VALID_REGEX.get().unwrap().split(
                    x.as_ref()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
            ).filter(|y| !y.is_empty()).collect::<Vec<&str>>().len()
        ).collect::<Vec<usize>>().iter().sum::<usize>();

    let syll_out: usize = valid_parts + add_counter - sub_counter;

    if syll_out <= 0 {
        return 1;
    } else {
        return syll_out;
    }
}
