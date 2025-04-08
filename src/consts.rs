pub mod msg {
    pub const FAILED_CONVERTING_TO_INDEPENDENT_BITS:&str = "[Panicked from cicaklang's source code]\nCicak Runtime: Failed to handle larger bits to machine's bits. This can be caused that the machine dosent support 32bits (maybe 16bits). 
\nAlso Cicaklang dosent support 16bit platforms for the runtime, consider compile the program instead";
}
