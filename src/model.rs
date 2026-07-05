#[derive(Clone, Debug, Default)]
pub struct Archive {
    pub header: Header,
    pub dictionary: Dictionary,
    pub topology: Topology,
    pub sessions: Vec<Session>,
    pub journal: Vec<JournalEntry>,
    pub scripts: Vec<Program>,
    pub blobs: Vec<Vec<u8>>,
    pub diagnostics: Vec<String>,
}

impl Archive {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            ..Self::default()
        }
    }

    pub fn frame_count(&self) -> usize {
        self.sessions
            .iter()
            .map(|session| session.frames.len())
            .sum()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Header {
    pub version: u8,
    pub flags: u8,
    pub created_at: u64,
    pub yard_id: u32,
    pub section_count: u16,
}

#[derive(Clone, Debug)]
pub struct Section {
    pub kind: u8,
    pub flags: u8,
    pub offset: usize,
    pub len: usize,
    pub checksum: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Dictionary {
    pub symbols: Vec<Symbol>,
    pub alias_score: u64,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub id: u32,
    pub kind: SymbolKind,
    pub text: String,
    pub score: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Track,
    Signal,
    Switch,
    Route,
    Crew,
    Zone,
    Script,
    WorkOrder,
    Unknown(u8),
}

impl SymbolKind {
    pub fn code(self) -> u8 {
        match self {
            SymbolKind::Track => 1,
            SymbolKind::Signal => 2,
            SymbolKind::Switch => 3,
            SymbolKind::Route => 4,
            SymbolKind::Crew => 5,
            SymbolKind::Zone => 6,
            SymbolKind::Script => 7,
            SymbolKind::WorkOrder => 8,
            SymbolKind::Unknown(v) => v,
        }
    }
}

impl Default for SymbolKind {
    fn default() -> Self {
        SymbolKind::Unknown(0)
    }
}

impl From<u8> for SymbolKind {
    fn from(value: u8) -> Self {
        match value {
            1 => SymbolKind::Track,
            2 => SymbolKind::Signal,
            3 => SymbolKind::Switch,
            4 => SymbolKind::Route,
            5 => SymbolKind::Crew,
            6 => SymbolKind::Zone,
            7 => SymbolKind::Script,
            8 => SymbolKind::WorkOrder,
            other => SymbolKind::Unknown(other),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Topology {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub signals: Vec<Signal>,
    pub switches: Vec<Switch>,
    pub zones: Vec<Zone>,
}

#[derive(Clone, Debug, Default)]
pub struct Node {
    pub id: u32,
    pub kind: u8,
    pub x: i16,
    pub y: i16,
    pub zone: u16,
    pub name_id: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Edge {
    pub id: u32,
    pub from: u32,
    pub to: u32,
    pub length: u16,
    pub grade: i16,
    pub flags: u8,
}

#[derive(Clone, Debug, Default)]
pub struct Signal {
    pub id: u32,
    pub node: u32,
    pub aspect: u8,
    pub route: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Switch {
    pub id: u32,
    pub node: u32,
    pub normal_edge: u32,
    pub reverse_edge: u32,
    pub locked: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Zone {
    pub id: u16,
    pub name_id: u32,
    pub risk: u8,
    pub max_speed: u16,
}

#[derive(Clone, Debug, Default)]
pub struct Session {
    pub id: u32,
    pub started_at: u64,
    pub frames: Vec<Frame>,
    pub checksum: u64,
}

#[derive(Clone, Debug)]
pub enum Frame {
    Track {
        track_id: u32,
        occupied: bool,
        confidence: u8,
        at: u64,
    },
    Switch {
        switch_id: u32,
        target: u8,
        locked: bool,
        at: u64,
    },
    Route {
        route_id: u32,
        start: u32,
        end: u32,
        authority: u8,
        at: u64,
    },
    Signal {
        signal_id: u32,
        aspect: u8,
        cause: u16,
        at: u64,
    },
    WorkOrder {
        order_id: u32,
        priority: u8,
        text: String,
        at: u64,
    },
    Sensor {
        track_id: u32,
        readings: Vec<i16>,
        at: u64,
    },
    Heartbeat {
        status: u32,
        at: u64,
    },
    Unknown {
        kind: u8,
        payload: Vec<u8>,
        at: u64,
    },
}

impl Frame {
    pub fn at(&self) -> u64 {
        match self {
            Frame::Track { at, .. }
            | Frame::Switch { at, .. }
            | Frame::Route { at, .. }
            | Frame::Signal { at, .. }
            | Frame::WorkOrder { at, .. }
            | Frame::Sensor { at, .. }
            | Frame::Heartbeat { at, .. }
            | Frame::Unknown { at, .. } => *at,
        }
    }

    pub fn code(&self) -> u8 {
        match self {
            Frame::Track { .. } => 1,
            Frame::Switch { .. } => 2,
            Frame::Route { .. } => 3,
            Frame::Signal { .. } => 4,
            Frame::WorkOrder { .. } => 5,
            Frame::Sensor { .. } => 6,
            Frame::Heartbeat { .. } => 7,
            Frame::Unknown { kind, .. } => *kind,
        }
    }
}

#[derive(Clone, Debug)]
pub struct JournalEntry {
    pub kind: u8,
    pub key: u32,
    pub clock: u64,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub version: u8,
    pub locals: Vec<i64>,
    pub instructions: Vec<Instruction>,
    pub symbols: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Instruction {
    pub opcode: u8,
    pub operand: i32,
    pub line: u32,
}

#[derive(Clone, Debug, Default)]
pub struct AnalysisReport {
    pub findings: Vec<AnalysisFinding>,
    pub checksum: u64,
    pub frame_count: usize,
    pub topology_score: u64,
    pub journal_score: u64,
    pub script_score: u64,
}

#[derive(Clone, Debug)]
pub struct AnalysisFinding {
    pub severity: u8,
    pub code: String,
    pub message: String,
}
