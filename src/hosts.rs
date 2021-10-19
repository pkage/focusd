use std::fs::File;
use std::io::{BufReader, BufRead, Error};

static FOCUSD_BLOCK_START: &'static str = "# --- focusd start ---";
static FOCUSD_BLOCK_END:   &'static str = "# --- focusd end ---";

pub fn hosts_remove(hostsfile: &String) -> Result<String, Error> {
    let mut output: Vec<String> = Vec::new();

    let input = File::open(hostsfile)?;
    let buffered = BufReader::new(input);

    let mut focusd_block = false;

    for perhapsline in buffered.lines() {
        let line = perhapsline?;

        // watch for control lines
        if line == FOCUSD_BLOCK_START {
            focusd_block = true;
        } else if line == FOCUSD_BLOCK_END {
            focusd_block = false;
        } else {
            if !focusd_block {
                output.push(line);
            }
            
        }
    }

    Ok(output.join("\n"))
}

pub fn hosts_add(hostsfile: &String, blocked: &Vec<String>) -> Result<String, Error> {
    let mut output: Vec<String> = Vec::new();

    // read in the hostfile
    let hosts = File::open(hostsfile)?;
    let buffered = BufReader::new(hosts);

    for perhapsline in buffered.lines() {
        let line = perhapsline?;
        output.push(line)
    }

    // add the header
    output.push(FOCUSD_BLOCK_START.to_string());

    // attempt to load in the configuration
    for site in blocked.iter() {
        output.push(format!("127.0.0.1\t{}", site));
    }

    // add the footer
    output.push(FOCUSD_BLOCK_END.to_string());

    Ok(output.join("\n"))
}

pub fn hosts_active(hostsfile: &String) -> Result<bool, Error> {
    // read in the hostfile
    let hosts = File::open(hostsfile)?;
    let buffered = BufReader::new(hosts);

    for perhapsline in buffered.lines() {
        let line = perhapsline?;
        if line == FOCUSD_BLOCK_START {
            return Ok(true);
        }
    }

    return Ok(false);
}
