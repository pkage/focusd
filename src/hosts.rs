use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use super::config::*;

pub fn hosts_remove(hostsfile: &String) -> Result<String, Error> {
    let mut output: Vec<String> = Vec::new();

    let input = File::open(hostsfile)?;
    let buffered = BufReader::new(input);

    let mut focusd_block = false;

    for perhapsline in buffered.lines() {
        let line = perhapsline?;

        // watch for control lines
        if line == "# --- focusd start ---" {
            focusd_block = true;
        } else if line == "# --- focusd end ---" {
            focusd_block = false;
        } else {
            if !focusd_block {
                output.push(line);
            }
            
        }
    }

    Ok(output.join("\n"))
}

pub fn hosts_add(hostsfile: &String, config: &FocusConfig) -> Result<String, Error> {
    let mut output: Vec<String> = Vec::new();

    // read in the hostfile
    let hosts = File::open(hostsfile)?;
    let buffered = BufReader::new(hosts);

    for perhapsline in buffered.lines() {
        let line = perhapsline?;
        output.push(line)
    }

    // attempt to load in the configuration


    Ok(output.join("\n"))
}
