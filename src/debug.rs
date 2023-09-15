use std::fmt::Display;

#[macro_export]
macro_rules! debug {
    ( $dbg_stmt:stmt ) => {
        if std::env::var("DEBUG").is_ok() {
            $dbg_stmt
        }
    };
}

pub enum Step {
	// CIPHER
	Input,
	Start,
	SubBytes,
	ShiftRows,
	MixColumns,
	KeySchedule,
	Output,

	// INVERSE CIPHER
	IInput,
	IStart,
	ISubBytes,
	IShiftRows,
	IKeySchedule,
	IAddRoundKey,
	IOutput,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let string = match self {
			Self::Input => "input",
			Self::Start => "start",
			Self::SubBytes => "s_box",
			Self::ShiftRows => "s_row",
			Self::MixColumns => "m_col",
			Self::KeySchedule => "k_sch",
			Self::Output => "output",
			Self::IInput => "iinput",
			Self::IStart => "istart",
			Self::ISubBytes => "is_box",
			Self::IShiftRows => "is_row",
			Self::IKeySchedule => "ik_sch",
			Self::IAddRoundKey => "ik_add",
			Self::IOutput => "ioutput",
		};
        write!(f, "{string}")
    }
}

pub fn print_state<S, T>(round: usize, step: Step, state: S)
where
    S: AsRef<[T]>,
    T: AsRef<[u8]>,
{
    let state = state_to_hex_string(state);
    println!("round[{round:2}].{step:8} {state}");
}

fn state_to_hex_string<S, T>(state: S) -> String
where
    S: AsRef<[T]>,
    T: AsRef<[u8]>,
{
    state
        .as_ref()
        .iter()
        .map(|row| {
            row.as_ref()
                .iter()
                .map(|value| format!("{:02x}", value))
                .collect::<String>()
        })
        .collect()
}

pub fn print_key_sched<S>(round: usize, step: Step, key_sched: S)
where S: AsRef<[u32]> {
	let key_sched = key_sched_to_hex_string(key_sched);
    println!("round[{round:2}].{step:8} {key_sched}");
}

fn key_sched_to_hex_string<S>(key_sched: S) -> String
where S: AsRef<[u32]> {
	key_sched.as_ref().iter().map(|word| format!("{:08x}", word)).collect()
}
