/*****************************************************************************/
/*****************************************************************************/
/*****************************************************************************/
/**                                                                         **/
/**   Name: rust-txt2tags-manifestation-block-generator                     **/
/**   Description: [Endless Return] Generate a standard Sign block for Sign **/
/**     abilities.                                                          **/
/**   Updated: 2022-04-10                                                   **/
/**                                                                         **/
/*****************************************************************************/
/*****************************************************************************/
/*****************************************************************************/

// Design Notes.
// TODO: Have the script write out generated Sign blocks to a file.
// TODO: Separate the various pieces of functionality into individual
//      functions.
// TODO: Collapse the code footprint where possible.
// TODO: Surface this functionality as a class/object/library for usage in
//      more robust content-generation scripts developed later.

// Imports.
use std::collections::HashMap;
use std::io::*;

// Entrypoint.
fn main() {
    // Variables.
    let sign_field_vector = vec!["sign_name", "tier", "cost", "action_type", "beat_cost", "duration", "intensity", "markings", "rule_block"];
    let prompt_vector = vec!["the name of the Sign", "the tier of the Sign", "the Cost of the Sign", "the type of Action", "the Beat cost (if applicable)", "the Duration", "the Intensity", "the Markings", "the rules text"];
    let output_vector = vec!["**__{}__**\n\n", "__Tier:__ {}\n\n", "__Cost:__ {}\n\n", "__Action Type:__ {}\n\n", "__Action Type:__ {} (<>bt)\n\n", "__Duration:__ {}\n\n", "__Intensity:__ {}\n\n", "__Markings:__ {}\n\n", "{}\n\n"];
    let sign_and_prompt_map: HashMap<_, _> =
        sign_field_vector.clone().into_iter().zip(prompt_vector.clone().into_iter()).collect();
    let mut sign_and_input_map = HashMap::new();
    let sign_and_output_map: HashMap<_, _> = 
        sign_field_vector.clone().into_iter().zip(output_vector.clone().into_iter()).collect();

    // Main Functionality.
    for &sign_field_token in &sign_field_vector {
        //println!("{} / {}", key, value);
        match sign_and_prompt_map.get(sign_field_token) {
            Some(prompt) => print!("PROMPT: Please provide {}. # ", prompt),
            None => println!("ERROR: {} isn't present in sign_and_prompt hashmap.", sign_field_token)
        }
        stdout().flush().unwrap();
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input.pop();
        sign_and_input_map.insert(
            sign_field_token,
            user_input
        );
    }
    
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    let mut i = 0;
    let mut sign_field_token;
    let mut _current_input: &str = &"";
    let mut _current_output: &str = &"";
    let mut _special_input: &str = &"";
    let mut _action_type_output: bool = false;
    while i < sign_field_vector.len() {
        sign_field_token = sign_field_vector[i];
        match sign_and_input_map.get(sign_field_token) {
            Some(input) => _current_input = input,
            None => println!("ERROR: {} isn't present in sign_and_input hashmap.", sign_field_token)
        }
        match sign_and_output_map.get(sign_field_token) {
            Some(output) => _current_output = output,
            None => println!("ERROR: {} isn't presnt in sign_and_output hashmap.", sign_field_token)
        }
        if sign_field_token != "action_type" && sign_field_token != "beat_cost" {
            //println!(output_vector[i], _current_input);
            lock.write_all(_current_output.replace("{}", _current_input).as_bytes()).unwrap();
            lock.flush().ok();
        } else {
            if _action_type_output == true {
                i = i + 1;
                continue;
            }
            _special_input = &"";
            _action_type_output = false;
            if sign_field_token == "action_type" {
                match sign_and_input_map.get("beat_cost") {
                    Some(input) => _special_input = input, 
                    None => ()
                }
            }
            if _special_input.trim().is_empty() == true {
                lock.write_all(_current_output.replace("{}", _current_input).as_bytes()).unwrap();
                lock.flush().ok();
            } else {
                match sign_and_output_map.get("beat_cost") {
                    Some(output) => lock.write_all(output.replace("{}", _current_input).replace("<>", _special_input).as_bytes()).unwrap(),
                    None => lock.write_all(b"").unwrap()
                }
                lock.flush().ok();
            }
            _action_type_output = true;
        }
        i = i + 1;
    }
}
