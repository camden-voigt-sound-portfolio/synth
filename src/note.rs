use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Note {
    A(i8),
    Bb(i8),
    B(i8),
    C(i8),
    Db(i8),
    D(i8),
    Eb(i8),
    E(i8),
    F(i8),
    Gb(i8),
    G(i8),
    Ab(i8),
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Note::A(n) => write!(f, "A{}", n),
            Note::Bb(n) => write!(f, "A#/Bb{}", n),
            Note::B(n) => write!(f, "B{}", n),
            Note::C(n) => write!(f, "C{}", n),
            Note::Db(n) => write!(f, "C#/Db{}", n),
            Note::D(n) => write!(f, "D{}", n),
            Note::Eb(n) => write!(f, "D#/Eb{}", n),
            Note::E(n) => write!(f, "E{}", n),
            Note::F(n) => write!(f, "F{}", n),
            Note::Gb(n) => write!(f, "F#/Gb{}", n),
            Note::G(n) => write!(f, "G{}", n),
            Note::Ab(n) => write!(f, "G#/Ab{}", n),
        }
    }
}

impl From<String> for Note {
    fn from(k: String) -> Self {
        match k.as_str() {
            "A3" => Note::A(3),
            "Bb3" | "A#3" => Note::Bb(3),
            "B3" => Note::B(3),
            "C4" => Note::C(4),
            "Db4" | "C#4" => Note::Db(4),
            "D4" => Note::D(4),
            "Eb4" | "D#4" => Note::Eb(4),
            "E4" => Note::E(4),
            "F4" => Note::F(4),
            "Gb4" | "F#4" => Note::Gb(4),
            "G4" => Note::G(4),
            "Ab4" | "G#4" => Note::Eb(4),
            "A4" => Note::A(4),
            _ => Note::A(3),
        }
    }
}

impl From<u8> for Note {
    fn from(n: u8) -> Self {
        let octave = (n / 12) as i8 - 1;
        match n % 12 {
            0 => Note::C(octave),
            1 => Note::Db(octave),
            2 => Note::D(octave),
            3 => Note::Eb(octave),
            4 => Note::E(octave),
            5 => Note::F(octave),
            6 => Note::Gb(octave),
            7 => Note::G(octave),
            8 => Note::Ab(octave),
            9 => Note::A(octave),
            10 => Note::Bb(octave),
            11 => Note::B(octave),
            _ => Note::C(0),
        }
    }
}

impl Note {
    pub fn freq(note: Note) -> f32 {
        // From https://inspiredacoustics.com/en/MIDI_note_numbers_and_center_frequencies
        match note {
            Note::A(-1) => 13.75,
            Note::A(n) => 2.0 * Note::freq(Note::A(n - 1)),
            Note::Bb(-1) => 14.57,
            Note::Bb(n) => 2.0 * Note::freq(Note::Bb(n - 1)),
            Note::B(-1) => 15.43,
            Note::B(n) => 2.0 * Note::freq(Note::B(n - 1)),
            Note::C(-1) => 8.18,
            Note::C(n) => 2.0 * Note::freq(Note::C(n - 1)),
            Note::Db(-1) => 8.66,
            Note::Db(n) => 2.0 * Note::freq(Note::Db(n - 1)),
            Note::D(-1) => 9.18,
            Note::D(n) => 2.0 * Note::freq(Note::D(n - 1)),
            Note::Eb(-1) => 9.72,
            Note::Eb(n) => 2.0 * Note::freq(Note::Eb(n - 1)),
            Note::E(-1) => 10.3,
            Note::E(n) => 2.0 * Note::freq(Note::E(n - 1)),
            Note::F(-1) => 10.91,
            Note::F(n) => 2.0 * Note::freq(Note::F(n - 1)),
            Note::Gb(-1) => 11.56,
            Note::Gb(n) => 2.0 * Note::freq(Note::Gb(n - 1)),
            Note::G(-1) => 12.25,
            Note::G(n) => 2.0 * Note::freq(Note::G(n - 1)),
            Note::Ab(-1) => 12.98,
            Note::Ab(n) => 2.0 * Note::freq(Note::Ab(n - 1)),
        }
    }

    pub fn midi_num(&self) -> u8 {
        let midi_num = match &self {
            Note::A(n) => (12 * (n + 1)) + 9,
            Note::Bb(n) => (12 * (n + 1)) + 10,
            Note::B(n) => (12 * (n + 1)) + 11,
            Note::C(n) => 12 * (n + 1),
            Note::Db(n) => (12 * (n + 1)) + 1,
            Note::D(n) => (12 * (n + 1)) + 2,
            Note::Eb(n) => (12 * (n + 1)) + 3,
            Note::E(n) => (12 * (n + 1)) + 4,
            Note::F(n) => (12 * (n + 1)) + 5,
            Note::Gb(n) => (12 * (n + 1)) + 6,
            Note::G(n) => (12 * (n + 1)) + 7,
            Note::Ab(n) => (12 * (n + 1)) + 8,
        };
        midi_num as u8
    }

    pub fn half_step(self) -> Note {
        match self {
            Note::A(n) => Note::Bb(n),
            Note::Bb(n) => Note::B(n),
            Note::B(n) => Note::C(n + 1),
            Note::C(n) => Note::Db(n),
            Note::Db(n) => Note::D(n),
            Note::D(n) => Note::Eb(n),
            Note::Eb(n) => Note::E(n),
            Note::E(n) => Note::F(n),
            Note::F(n) => Note::Gb(n),
            Note::Gb(n) => Note::G(n),
            Note::G(n) => Note::Ab(n),
            Note::Ab(n) => Note::A(n),
        }
    }

    pub fn whole_step(self) -> Note {
        self.half_step().half_step()
    }

    pub fn major_key(self) -> Key {
        let mut key = vec![self];

        for i in 1..8 {
            if i == 3 || i == 7 {
                key.push(key[i - 1].half_step());
            } else {
                key.push(key[i - 1].whole_step());
            }
        }

        key
    }

    #[allow(unused)]
    pub fn minor_key(self) -> Key {
        let mut key = vec![self];

        for i in 1..8 {
            if i == 2 || i == 5 {
                key.push(key[i - 1].half_step());
            } else {
                key.push(key[i - 1].whole_step());
            }
        }

        key
    }

    pub fn major_third(self) -> Note {
        self.whole_step().whole_step()
    }

    pub fn minor_third(self) -> Note {
        self.whole_step().half_step()
    }

    pub fn perfect_fifth(self) -> Note {
        self.whole_step().whole_step().whole_step().half_step()
    }

    pub fn octave_drop(self) -> Note {
        match self {
            Note::A(n) => Note::A(n - 1),
            Note::Bb(n) => Note::Bb(n - 1),
            Note::B(n) => Note::B(n - 1),
            Note::C(n) => Note::C(n - 1),
            Note::Db(n) => Note::Db(n - 1),
            Note::D(n) => Note::D(n - 1),
            Note::Eb(n) => Note::Eb(n - 1),
            Note::E(n) => Note::E(n - 1),
            Note::F(n) => Note::F(n - 1),
            Note::Gb(n) => Note::Gb(n - 1),
            Note::G(n) => Note::G(n - 1),
            Note::Ab(n) => Note::Ab(n - 1),
        }
    }
}

pub type Key = Vec<Note>;

#[derive(PartialEq, Clone, Copy, Debug)]
#[allow(non_camel_case_types, unused, clippy::upper_case_acronyms)]
pub enum Chord {
    I,
    i,
    II,
    ii,
    III,
    iii,
    IV,
    iv,
    V,
    v,
    VI,
    vi,
    VII,
    vii,
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chord::I => write!(f, "I"),
            Chord::i => write!(f, "ii"),
            Chord::II => write!(f, "II"),
            Chord::ii => write!(f, "ii"),
            Chord::III => write!(f, "III"),
            Chord::iii => write!(f, "iii"),
            Chord::IV => write!(f, "IV"),
            Chord::iv => write!(f, "iv"),
            Chord::V => write!(f, "V"),
            Chord::v => write!(f, "v"),
            Chord::VI => write!(f, "VI"),
            Chord::vi => write!(f, "vi"),
            Chord::VII => write!(f, "VII"),
            Chord::vii => write!(f, "vii"),
        }
    }
}

impl Chord {
    pub fn notes(self, key: &Key) -> ChordNotes {
        let base_note = match self {
            Chord::I | Chord::i => key[0],
            Chord::II | Chord::ii => key[1],
            Chord::III | Chord::iii => key[2],
            Chord::IV | Chord::iv => key[3],
            Chord::V | Chord::v => key[4],
            Chord::VI | Chord::vi => key[5],
            Chord::VII | Chord::vii => key[6],
        };

        let mut third_note = match self {
            Chord::I | Chord::II | Chord::III | Chord::IV | Chord::V | Chord::VI | Chord::VII => {
                base_note.major_third()
            }
            Chord::i | Chord::ii | Chord::iii | Chord::iv | Chord::v | Chord::vi | Chord::vii => {
                base_note.minor_third()
            }
        };

        let mut fifth_note = base_note.perfect_fifth();

        if !key.contains(&third_note) {
            third_note = third_note.octave_drop();
        }

        if !key.contains(&fifth_note) {
            fifth_note = fifth_note.octave_drop();
        }

        vec![base_note, third_note, fifth_note]
    }
}

pub type ChordNotes = Vec<Note>;
