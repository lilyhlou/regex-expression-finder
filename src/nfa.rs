#![allow(unused)]
#![allow(non_snake_case)]
use std::ops::Add;

pub mod helpers;

// Starter code for PS06 - thegrep
use self::State::*;
/**
* thegrep - Tar Heel Extended Regular Expressions - NFA
*
* Author: <Taylor Montgomery, Lily Lou>
* ONYEN: <tayjomo, loulh>
*
* UNC Honor Pledge: I pledge I have received no unauthorized aid
* on this assignment. I further pledge not to distribute my solution
* to this code to anyone other than the course staff.
*/
use super::parser::Parser;
use super::parser::AST;
use super::tokenizer::Tokenizer;
use rand::distributions::Alphanumeric;
use rand::*;
use rand::{thread_rng, Rng};

/**
 * ===== Public API =====
 */

/**
 * An NFA is represented by an arena Vec of States
 * and a start state.
 */
#[derive(Debug)]
pub struct NFA {
    start: StateId,
    states: Vec<State>,
}

impl NFA {
    /**
     * Construct an NFA from a regular expression pattern.
     */
    pub fn from(regular_expression: &str) -> Result<NFA, String> {
        let mut nfa = NFA::new();

        let start = nfa.add_state(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add_state(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    /**
     * Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     */
    pub fn accepts(&self, input: &str) -> bool {
        let mut current = Vec::new(); // can be in multiple states at once
        let mut next_states = Vec::new(); // vec of ids of next states
        current.push(self.start);
        let mut input_string = input.chars(); // iterator for input characters
        while let Some(ch) = input_string.next() {
            // iterate through string and for each char, go through a list of states
            // and see if they match, send matches to next_states vector to parse through later
            &self.parse_state(&mut current, &mut next_states);
            while !current.is_empty() {
                match &self.states[current.remove(0)] {
                    State::Start(id) => {
                        let mut parse_start = Vec::new();
                        parse_start.push(id.unwrap());
                        self.parse_state(&mut current, &mut parse_start);
                    }
                    State::Match(match_char, id) => match match_char {
                        Char::Literal(match_character) => {
                            if match_character == &ch {
                                next_states.push(id.unwrap());
                            }
                        }
                        Char::Any => {
                            next_states.push(id.unwrap());
                        }
                    },
                    State::End => return true, // for parsing (string).*
                    _ => {}                    // should not hit
                }
            }
        }
        // parse through once more to see if there is an end state that wasn't reached
        // ex: "ab" for nfa::from("ab.*")
        &self.parse_state(&mut current, &mut next_states);
        while !current.is_empty() {
            match &self.states[current.remove(0)] {
                State::End => return true,
                _ => {}
            }
        }
        false
    }
    /**Given an NFA, generate strings accepted by the pattern
     */
    pub fn gen(&self) -> String {
        //vector for randome or literal characters
        let mut gen_str = Vec::new();

        //vector of next state indexes in NFA states
        let mut next_states = Vec::new();

        //populate next_states with start ID
        next_states.push(self.start);

        //while next states is not empty
        //use stateId to access the appropriate next state in self.states
        //next state ID will always be the first element of the vector
        //add next state to vector once matched
        while !next_states.is_empty() {
            match &self.states[next_states[0]] {
                State::Start(id) => next_states.push(id.unwrap()),
                State::Match(m_char, id) => {
                    //add char to vector, if anychar generate and add random char
                    next_states.push(id.unwrap());
                    match m_char {
                        Char::Literal(match_character) => gen_str.push(*match_character),
                        Char::Any => {
                            let mut rng = thread_rng();
                            let mut c = rng.sample(Alphanumeric);
                            gen_str.push(c);
                        }
                    }
                }

                State::Split(id1, id2) => {
                    //random bool will determine which split path to follow
                    let mut path = rand::random::<bool>();
                    if path {
                        next_states.push(id1.unwrap());
                    } else {
                        next_states.push(id2.unwrap());
                    }
                }

                State::End => {
                    break;
                }
            }

            //remove old stateID value to push new stateId to the left/0 position
            next_states.remove(0);
        }

        //collect gen_str vector to create string
        let mut complete_str: String = String::new();
        complete_str = gen_str.into_iter().collect();
        complete_str
    }
}

#[cfg(test)]
mod nfa {
    use super::*;
    mod nfa_accepts {
        use super::*;

        #[test]
        fn accept_simple_true() {
            let nfa = NFA::from("b").unwrap();
            assert_eq!(true, nfa.accepts("b"));

            let nfa = NFA::from("a").unwrap();
            assert_eq!(true, nfa.accepts("a"));

            let nfa = NFA::from("us").unwrap();
            assert_eq!(true, nfa.accepts("us"));
        }

        #[test]
        fn best_match() {
            let nfa = NFA::from(".*unc.*").unwrap();
            assert_eq!(true, nfa.accepts("unc"));
            assert_eq!(true, nfa.accepts("hellounc"));
            assert_eq!(true, nfa.accepts("bounce"));
            assert_eq!(true, nfa.accepts("uncork"));
            assert_eq!(true, nfa.accepts("lunch"));
            assert_eq!(true, nfa.accepts("munch"));
        }

        #[test]
        fn Nomatch_anyKleeneStar() {
            let nfa = NFA::from(".*a.*").unwrap();
            assert_eq!(false, nfa.accepts("cdc"));
            assert_eq!(false, nfa.accepts("dgf"));
            assert_eq!(false, nfa.accepts("bub"));
            assert_eq!(false, nfa.accepts("hik"));
        }

        #[test]
        fn accept_kleenestar() {
            let nfa = NFA::from(".*ab*.*").unwrap();
            assert_eq!(true, nfa.accepts("abb"));
            assert_eq!(true, nfa.accepts("abbbbbbbbbbbb"));
            assert_eq!(true, nfa.accepts("cab"));

            let nfa = NFA::from(".*abd*").unwrap();
            assert_eq!(true, nfa.accepts("abdd"));
            assert_eq!(true, nfa.accepts("abb"));
        }

        #[test]
        fn accepts_simple_false() {
            let nfa = NFA::from("abd").unwrap();
            assert_eq!(false, nfa.accepts("c"));
            let nfa = NFA::from(".*ab.*").unwrap();
            assert_eq!(false, nfa.accepts("hat"));
        }

        #[test]
        fn simple_alteration_kleene() {
            let nfa = NFA::from(".*(x|y)").unwrap();
            assert_eq!(true, nfa.accepts("x"));
            assert_eq!(true, nfa.accepts("y"));
            assert_eq!(true, nfa.accepts("ax"));
            assert_eq!(true, nfa.accepts("ay"));
        }

        #[test]
        fn accept_alteration_anychar() {
            let nfa = NFA::from("(a|b).d").unwrap();
            assert_eq!(true, nfa.accepts("bad"));
            assert_eq!(true, nfa.accepts("bud"));
            assert_eq!(false, nfa.accepts("bat"));
            assert_eq!(true, nfa.accepts("and"));
        }

        #[test]
        fn accept_anychar() {
            let nfa = NFA::from(".....").unwrap();
            assert_eq!(true, nfa.accepts("yikes"));
            assert_eq!(true, nfa.accepts("zoned"));
            assert_eq!(false, nfa.accepts("yay"));
            assert_eq!(true, nfa.accepts("alimony"));
            assert_eq!(false, nfa.accepts("oreo"));

            let nfa = NFA::from("...*").unwrap();
            assert_eq!(true, nfa.accepts("ah"));
        }

        #[test]
        fn accept_caten() {
            let nfa = NFA::from(".*us.*").unwrap();
            assert_eq!(true, nfa.accepts("transfuse"));
            assert_eq!(true, nfa.accepts("suspicion"));
            assert_eq!(true, nfa.accepts("use"));
            assert_eq!(false, nfa.accepts("super"));
            assert_eq!(false, nfa.accepts("happy"));
        }

        #[test]
        fn caten_Kleenstar_alter_lvl1() {
            let nfa = NFA::from(".*(a|b)*...g.*").unwrap();
            assert_eq!(true, nfa.accepts("ring"));
            assert_eq!(true, nfa.accepts("programmer"));
            assert_eq!(true, nfa.accepts("mythology"));
            assert_eq!(true, nfa.accepts("applegate"));
            assert_eq!(false, nfa.accepts("apple"));
        }

        #[test]
        fn nfa_gone_wild() {
            let nfa = NFA::from("(.*a.*.((aa)*b|(e|d)))|(x*h(i|o))").unwrap();
            assert_eq!(true, nfa.accepts("contemplate"));
            assert_eq!(true, nfa.accepts("convalescences"));
            assert_eq!(true, nfa.accepts("emulate"));
            assert_eq!(true, nfa.accepts("emphasize"));
            assert_eq!(false, nfa.accepts("goat"));
            assert_eq!(false, nfa.accepts("glass"));
            assert_eq!(false, nfa.accepts("easy"));
            assert_eq!(false, nfa.accepts("decimal"));
        }

        #[test]
        fn alternation_catenation() {
            let nfa = NFA::from(".*(t|k)(a|i)(b|d).*").unwrap();
            assert_eq!(true, nfa.accepts("tide"));
            assert_eq!(true, nfa.accepts("table"));
            assert_eq!(true, nfa.accepts("kid"));
            assert_eq!(true, nfa.accepts("tab"));
            assert_eq!(false, nfa.accepts("fast"));
            assert_eq!(false, nfa.accepts("act"));
            assert_eq!(false, nfa.accepts("the"));
        }

        #[test]
        fn alt_in_an_alt() {
            let nfa = NFA::from("(a|b)|(t|z)").unwrap();
            assert_eq!(true, nfa.accepts("bash"));
            assert_eq!(true, nfa.accepts("avocado"));
            assert_eq!(true, nfa.accepts("zebra"));
            assert_eq!(false, nfa.accepts("hello"));
            assert_eq!(false, nfa.accepts("unc"));
            assert_eq!(false, nfa.accepts("got"));
        }

        #[test]
        fn automata() {
            let nfa = NFA::from("aut....a").unwrap();
            assert_eq!(true, nfa.accepts("automata"));
        }

        #[test]
        fn kleeneplus() {
            let nfa = NFA::from("hel+o").unwrap();
            assert_eq!(true, nfa.accepts("helllllllllo"));
            assert_eq!(false, nfa.accepts("heo"));
        }

        #[test]
        fn add_overload() {
            let ab = NFA::from("ab").unwrap();
            let cd = NFA::from("cd").unwrap();
            let abcd = ab + cd;
            assert_eq!(true, abcd.accepts("abcd"));
        }

        #[test]
        fn overload_kleene() {
            let a_star = NFA::from("a*").unwrap();
            let b_star = NFA::from("b*").unwrap();
            let ab = a_star + b_star;
            assert!(ab.accepts("a"));
            assert!(ab.accepts("b"));
            assert!(ab.accepts("ab"));
            assert!(ab.accepts("aabbb"));
        }
    }

    mod generate {
        use super::*;

        #[test]
        fn gen_cat() {
            let nfa = NFA::from(".*ab.*").unwrap();
            let gen_str = nfa.gen();
            assert!(nfa.accepts(&format!("{}", gen_str)));
        }

        #[test]
        fn gen_alt() {
            let nfa = NFA::from(".*cab|t|kl.*").unwrap();
            let gen_str = nfa.gen();
            assert!(nfa.accepts(&format!("{}", gen_str)));
        }

        #[test]
        fn gen_kleenestar() {
            let nfa = NFA::from(".ha*t").unwrap();
            let gen_str = nfa.gen();
            assert!(nfa.accepts(&format!("{}", gen_str)));
        }

        #[test]
        fn gen_kleeneplus() {
            let nfa = NFA::from(".*ab.+").unwrap();
            let gen_str = nfa.gen();
            assert!(nfa.accepts(&format!("{}", gen_str)));
        }

        #[test]
        fn gen_any() {
            let nfa = NFA::from("........").unwrap();
            let gen_str = nfa.gen();
            assert!(nfa.accepts(&format!("{}", gen_str)));
        }

        #[test]
        fn gen_anykleene() {
            let nfa = NFA::from(".*.+").unwrap();
            let gen_str = nfa.gen();
            assert!(nfa.accepts(&format!("{}", gen_str)));
        }

        #[test]
        fn gen_combo() {
            let nfa = NFA::from("(.+a.*.+((aa)*b|(e|d+)))|(x+h(i*|o*))").unwrap();
            let gen_str = nfa.gen();
            assert!(nfa.accepts(&format!("{}", gen_str)));
        }

    }
}

/**
 * ===== Internal API =====
 */
type StateId = usize;

/**
 * States are the elements of our NFA Graph
 * - Start is starting state
 * - Match is a state with a single matching transition out
 * - Split is a state with two epsilon transitions out
 * - End is the final accepting state
 */
#[derive(Debug)]
enum State {
    Start(Option<StateId>),
    Match(Char, Option<StateId>),
    Split(Option<StateId>, Option<StateId>),
    End,
}

/**
 * Chars are the matching label of a non-epsilon edge in the
 * transition diagram representation of the NFA.
 */
#[derive(Debug)]
enum Char {
    Literal(char),
    Any,
}

/**
 * Internal representation of a fragment of an NFA being constructed
 * that keeps track of the start ID of the fragment as well as all of
 * its unjoined end states.
 */
#[derive(Debug)]
struct Fragment {
    start: StateId,
    ends: Vec<StateId>,
}

/**
 * Private methods of the NFA structure.
 */
impl NFA {
    /**
     * Constructor establishes an empty states Vec.
     */
    fn new() -> NFA {
        NFA {
            states: vec![],
            start: 0,
        }
    }

    /**
     * Add a state to the NFA and get its arena ID back.
     */
    fn add_state(&mut self, state: State) -> StateId {
        let idx = self.states.len();
        self.states.push(state);
        idx
    }

    /**
     * Given an AST node, this method returns a Fragment of the NFA
     * representing it and its children.
     */
    fn gen_fragment(&mut self, ast: &AST) -> Fragment {
        match ast {
            AST::AnyChar => {
                let state = self.add_state(Match(Char::Any, None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Char(c) => {
                let state = self.add_state(Match(Char::Literal(*c), None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Catenation(lhs, rhs) => {
                let left = self.gen_fragment(lhs);
                let right = self.gen_fragment(rhs);
                self.join_fragment(&left, right.start);
                Fragment {
                    start: left.start,
                    ends: right.ends,
                }
            }
            AST::Alternation(lhs, rhs) => {
                let left = self.gen_fragment(lhs);
                let right = self.gen_fragment(rhs);
                let splitstate = self.add_state(Split(Some(left.start), Some(right.start)));
                Fragment {
                    start: splitstate,
                    ends: [left.ends.as_slice(), right.ends.as_slice()].concat(),
                }
            }
            AST::Closure(lhs) => {
                let kleenestar = self.gen_fragment(lhs);
                let split = self.add_state(Split(Some(kleenestar.start), None));
                // want kleenestar (the pattern repeated) to point back to split
                self.join_fragment(&kleenestar, split);
                Fragment {
                    start: split,
                    ends: vec![split],
                }
            }
            AST::OneOrMore(lhs) => {
                let kleeneplus = self.gen_fragment(lhs);
                let split = self.add_state(Split(Some(kleeneplus.start), None));
                self.join_fragment(&kleeneplus, split);
                Fragment {
                    start: kleeneplus.start,
                    ends: vec![split],
                }
            }
            node => panic!("Unimplemented branch of gen_fragment: {:?}", node),
        }
    }

    /**
     * Join all the loose ends of a fragment to another StateId.
     */
    fn join_fragment(&mut self, lhs: &Fragment, to: StateId) {
        for end in &lhs.ends {
            self.join(*end, to);
        }
    }

    /**
     * Join a loose end of one state to another by IDs.
     * Note in the Split case, only the 2nd ID (rhs) is being bound.
     * It is assumed when building an NFA with these constructs
     * that the lhs of an Split state will always be known and bound.
     */
    fn join(&mut self, from: StateId, to: StateId) {
        match self.states[from] {
            Start(ref mut next) => *next = Some(to),
            Match(_, ref mut next) => *next = Some(to),
            Split(_, ref mut next) => *next = Some(to),
            End => {}
        }
    }

    fn parse_state(&self, mut current: &mut Vec<StateId>, mut next_states: &mut Vec<StateId>) {
        // recurse a list of ids from split states until it hits a match or end state
        // parse_split takes in vectors and returns updated versions of them
        // split_states is a list of the StateIDs states point to, NOT actual split states
        // return the vector with next
        for next_id in (&next_states).iter() {
            match &self.states[*next_id] {
                State::Start(id) => {
                    current.push(*next_id);
                }
                State::Match(match_char, id) => {
                    current.push(*next_id);
                }
                State::Split(id1, id2) => {
                    let mut left_tree_splits = Vec::new();
                    left_tree_splits.push(id1.unwrap());
                    &self.parse_state(&mut current, &mut left_tree_splits);
                    let mut right_tree_splits = Vec::new();
                    right_tree_splits.push(id2.unwrap());
                    &self.parse_state(&mut current, &mut right_tree_splits);
                }
                State::End => {
                    current.push(*next_id);
                }
            }
        }
        next_states.clear();
    }
}

impl Add for NFA {
    type Output = NFA;

    fn add(self, rhs: NFA) -> NFA {
        let mut added_states = Vec::new();
        // clone left hand side (self)
        // end index is now start of second NFA, if start is 0, then it is now the index of the end
        // state, which is the size - 1
        // joint NFA will include start state from RHS
        // add rhs to NFA array
        for state in self.states {
            match state {
                State::Start(id) => added_states.push(State::Start(id)),
                State::Match(c, id) => added_states.push(State::Match(c, id)),
                State::Split(id1, id2) => added_states.push(State::Split(id1, id2)),
                State::End => {} // do not add
            }
        }

        let end_id = added_states.len(); // end is always last element on vector
        for rhstate in rhs.states {
            match rhstate {
                State::Start(id) => {
                    let rhs_id = id.unwrap() + end_id;
                    added_states.push(State::Start(Some(rhs_id)));
                }
                State::Match(c, id) => {
                    let rhs_id = id.unwrap() + end_id;
                    added_states.push(State::Match(c, Some(rhs_id)));
                }
                State::Split(id1, id2) => {
                    let rhs_id1 = id1.unwrap() + end_id;
                    let rhs_id2 = id2.unwrap() + end_id;
                    added_states.push(State::Split(Some(rhs_id1), Some(rhs_id2)));
                }
                State::End => added_states.push(State::End),
            }
        }
        NFA {
            start: 0,
            states: added_states,
        }
    }
}
