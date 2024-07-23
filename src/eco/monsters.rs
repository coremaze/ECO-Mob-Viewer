#[derive(Debug)]
pub struct Monster {
    pub name: &'static str,
    pub id: u32,
    pub monster_type: &'static str,
    pub level: u32,
    pub scale: f32,
    pub exp: u32,
    pub job: u32,
    pub hp: u32,
    pub sp: u32,
    pub mp: u32,
    pub def: [u32; 2],
    pub mdef: [u32; 2],
    pub properties: Properties,
    pub drop_ids: [u32; 8],
}

#[derive(Debug)]
pub struct Properties {
    pub fire: u32,
    pub water: u32,
    pub wind: u32,
    pub earth: u32,
    pub light: u32,
    pub dark: u32,
}

const MONSTERS: [Monster; 1507] = [
    Monster {
        name: "Pururu",
        id: 10000000,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 10049000, 10034507, 10034508, 10050900,
        ],
    },
    Monster {
        name: "Mini Pururu",
        id: 10000002,
        monster_type: "PLANT",
        level: 0,
        scale: 0.6,
        exp: 0,
        job: 0,
        hp: 21,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mini Mini Pururu",
        id: 10000003,
        monster_type: "PLANT_NOTOUCH",
        level: 1,
        scale: 0.4,
        exp: 0,
        job: 0,
        hp: 21,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Attack Sandbag",
        id: 10000004,
        monster_type: "PLANT",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [50, 50],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Jumbo Pururu",
        id: 10000006,
        monster_type: "PLANT_BOSS",
        level: 28,
        scale: 7.0,
        exp: 4500,
        job: 6750,
        hp: 10000,
        sp: 999,
        mp: 999,
        def: [30, 50],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001805, 10001805, 10032803, 10032803, 10032803, 10034507, 10005200, 0,
        ],
    },
    Monster {
        name: "Aqua Marine Pururu",
        id: 10000007,
        monster_type: "PLANT_UNITE",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10011100, 10011100, 10011100, 10011100, 0, /*null*/
            10011100, 10011100, 10050900,
        ],
    },
    Monster {
        name: "Oker Jelly",
        id: 10000100,
        monster_type: "PLANT",
        level: 21,
        scale: 1.0,
        exp: 216,
        job: 279,
        hp: 210,
        sp: 0,
        mp: 0,
        def: [10, 20],
        mdef: [10, 58],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10034507, 10034508, 0, /*null*/
            10032801, 10034505, 10050908,
        ],
    },
    Monster {
        name: "Infinite Jelly",
        id: 10000101,
        monster_type: "PLANT",
        level: 9,
        scale: 1.0,
        exp: 15,
        job: 15,
        hp: 60,
        sp: 1000,
        mp: 1000,
        def: [10, 16],
        mdef: [15, 22],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Ruby Pururu",
        id: 10000102,
        monster_type: "PLANT",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 50,
        sp: 0,
        mp: 0,
        def: [96, 300],
        mdef: [96, 300],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10011100, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10050900,
        ],
    },
    Monster {
        name: "Mage Pururu",
        id: 10000200,
        monster_type: "PLANT",
        level: 15,
        scale: 1.0,
        exp: 105,
        job: 123,
        hp: 110,
        sp: 44,
        mp: 49,
        def: [12, 16],
        mdef: [45, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002202, 10032800, 10032802, 10049000, 10034508, 10034505, 10050907,
        ],
    },
    Monster {
        name: "Magic Sandbag",
        id: 10000201,
        monster_type: "PLANT",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [50, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sweeper",
        id: 10000202,
        monster_type: "ELEMENT",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 120,
        sp: 44,
        mp: 49,
        def: [1, 10],
        mdef: [3, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Cool",
        id: 10000300,
        monster_type: "PLANT",
        level: 10,
        scale: 1.0,
        exp: 60,
        job: 63,
        hp: 75,
        sp: 0,
        mp: 0,
        def: [18, 23],
        mdef: [30, 27],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000210, 10001800, 10019900, 10049000, 10032804, 10034507, 10050903,
        ],
    },
    Monster {
        name: "Frozen Pururu",
        id: 10000301,
        monster_type: "PLANT",
        level: 42,
        scale: 1.2,
        exp: 1620,
        job: 2061,
        hp: 900,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000210, 10001800, 10019900, 0, /*null*/
            10032804, 10034507, 0,
        ],
    },
    Monster {
        name: "Milk Pitcher",
        id: 10000400,
        monster_type: "PLANT",
        level: 6,
        scale: 1.0,
        exp: 33,
        job: 42,
        hp: 41,
        sp: 0,
        mp: 0,
        def: [5, 7],
        mdef: [8, 30],
        properties: Properties {
            fire: 0,
            water: 10,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002200, 10008450, 90000045, 10032811, 10019900, 90000045, 10050901,
        ],
    },
    Monster {
        name: "Infinite Milk Pitcher",
        id: 10000401,
        monster_type: "PLANT",
        level: 18,
        scale: 1.0,
        exp: 36,
        job: 36,
        hp: 322,
        sp: 1000,
        mp: 1000,
        def: [10, 47],
        mdef: [10, 61],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Green Pururu",
        id: 10000500,
        monster_type: "PLANT",
        level: 12,
        scale: 1.0,
        exp: 69,
        job: 78,
        hp: 80,
        sp: 0,
        mp: 0,
        def: [7, 13],
        mdef: [15, 39],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10003200, 90000045, 10032805, 10003200, 10005200, 10050906,
        ],
    },
    Monster {
        name: "Infinite Green Pururu",
        id: 10000501,
        monster_type: "PLANT",
        level: 38,
        scale: 1.0,
        exp: 360,
        job: 270,
        hp: 850,
        sp: 1000,
        mp: 1000,
        def: [10, 59],
        mdef: [10, 61],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Emerald Pururu",
        id: 10000502,
        monster_type: "PLANT",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10011100, 10014850, 0, /*null*/
            0, /*null*/
            0, /*null*/
            10050900,
        ],
    },
    Monster {
        name: "Yellow Pururu",
        id: 10000700,
        monster_type: "PLANT",
        level: 11,
        scale: 1.0,
        exp: 51,
        job: 87,
        hp: 78,
        sp: 40,
        mp: 35,
        def: [29, 24],
        mdef: [27, 38],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10034507, 10034507, 10032807, 10008450, 10032807, 10050904,
        ],
    },
    Monster {
        name: "Diamond Pururu",
        id: 10000701,
        monster_type: "PLANT",
        level: 10,
        scale: 10.0,
        exp: 0,
        job: 0,
        hp: 240000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10050900,
        ],
    },
    Monster {
        name: "Amber Jelly",
        id: 10000800,
        monster_type: "PLANT",
        level: 7,
        scale: 1.0,
        exp: 45,
        job: 36,
        hp: 60,
        sp: 30,
        mp: 15,
        def: [6, 4],
        mdef: [15, 23],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10009000, 90000043, 10032808, 10045300, 90000043, 10050902,
        ],
    },
    Monster {
        name: "Lava Pururu",
        id: 10000801,
        monster_type: "ROCK",
        level: 32,
        scale: 1.5,
        exp: 933,
        job: 765,
        hp: 560,
        sp: 100,
        mp: 100,
        def: [25, 114],
        mdef: [30, 84],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10034507, 10000202, 0, /*null*/
            10014854, 10034502, 10050980,
        ],
    },
    Monster {
        name: "Infinite Larva Pururu",
        id: 10000802,
        monster_type: "PLANT",
        level: 51,
        scale: 1.5,
        exp: 675,
        job: 1125,
        hp: 1800,
        sp: 100,
        mp: 100,
        def: [25, 135],
        mdef: [20, 150],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Topaz Pururu",
        id: 10000803,
        monster_type: "PLANT",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10011100, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10050900,
        ],
    },
    Monster {
        name: "Coal Tar",
        id: 10000900,
        monster_type: "PLANT",
        level: 11,
        scale: 1.0,
        exp: 87,
        job: 51,
        hp: 75,
        sp: 16,
        mp: 17,
        def: [17, 19],
        mdef: [24, 34],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000701, 10035300, 10032809, 0, /*null*/
            10034508, 10034505, 10050905,
        ],
    },
    Monster {
        name: "Oily",
        id: 10000901,
        monster_type: "PLANT",
        level: 28,
        scale: 1.0,
        exp: 423,
        job: 387,
        hp: 380,
        sp: 999,
        mp: 999,
        def: [10, 42],
        mdef: [42, 74],
        properties: Properties {
            fire: 40,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000701, 10035300, 10032809, 0, /*null*/
            10034508, 10034505, 0,
        ],
    },
    Monster {
        name: "Grutness Pururu",
        id: 10000903,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 25,
        scale: 1.8,
        exp: 0,
        job: 0,
        hp: 150,
        sp: 16,
        mp: 17,
        def: [5, 20],
        mdef: [10, 34],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031161, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Rotting Pururu",
        id: 10000904,
        monster_type: "UNDEAD",
        level: 40,
        scale: 1.2,
        exp: 1440,
        job: 1575,
        hp: 1300,
        sp: 999,
        mp: 999,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10000701, 10035300, 10032809, 0, /*null*/
            10034508, 10034505, 0,
        ],
    },
    Monster {
        name: "Do Pururu",
        id: 10000950,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10015251, 10015251, 10015251, 10015251, 10015251, 10015251, 10015251, 0,
        ],
    },
    Monster {
        name: "Re Pururu",
        id: 10000951,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10015252, 10015252, 10015252, 10015252, 10015252, 10015252, 10015252, 0,
        ],
    },
    Monster {
        name: "Mi Pururu",
        id: 10000952,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10015253, 10015253, 10015253, 10015253, 10015253, 10015253, 10015253, 0,
        ],
    },
    Monster {
        name: "Fa Pururu",
        id: 10000953,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10015254, 10015254, 10015254, 10015254, 10015254, 10015254, 10015254, 0,
        ],
    },
    Monster {
        name: "So Pururu",
        id: 10000954,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10015255, 10015255, 10015255, 10015255, 10015255, 10015255, 10015255, 0,
        ],
    },
    Monster {
        name: "La Pururu",
        id: 10000955,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10015256, 10015256, 10015256, 10015256, 10015256, 10015256, 10015256, 0,
        ],
    },
    Monster {
        name: "Si Pururu",
        id: 10000956,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10015257, 10015257, 10015257, 10015257, 10015257, 10015257, 10015257, 0,
        ],
    },
    Monster {
        name: "Infinite Oily",
        id: 10000902,
        monster_type: "PLANT",
        level: 52,
        scale: 1.0,
        exp: 1125,
        job: 810,
        hp: 1900,
        sp: 999,
        mp: 999,
        def: [20, 140],
        mdef: [20, 188],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Metallic",
        id: 10001000,
        monster_type: "PLANT",
        level: 40,
        scale: 1.0,
        exp: 3498,
        job: 348,
        hp: 25,
        sp: 58,
        mp: 97,
        def: [96, 300],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015600, 10015710, 10015300, 10049000, 10015707, 10013300, 10050968,
        ],
    },
    Monster {
        name: "Lost Metallic",
        id: 10001001,
        monster_type: "PLANT",
        level: 24,
        scale: 1.0,
        exp: 1500,
        job: 150,
        hp: 14,
        sp: 58,
        mp: 97,
        def: [95, 240],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015600, 10015710, 10015300, 10049000, 10015707, 10013300, 10050909,
        ],
    },
    Monster {
        name: "Mini Metallic",
        id: 10001002,
        monster_type: "PLANT",
        level: 15,
        scale: 0.6,
        exp: 348,
        job: 33,
        hp: 8,
        sp: 58,
        mp: 97,
        def: [95, 300],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015600, 10015710, 10015300, 10049000, 10015707, 10013300, 0,
        ],
    },
    Monster {
        name: "Infinite Metallic",
        id: 10001003,
        monster_type: "PLANT",
        level: 55,
        scale: 1.0,
        exp: 1500,
        job: 150,
        hp: 50,
        sp: 58,
        mp: 97,
        def: [100, 400],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Maze Metallic",
        id: 10001004,
        monster_type: "PLANT_SKILL",
        level: 40,
        scale: 1.1,
        exp: 15000,
        job: 15000,
        hp: 100,
        sp: 58,
        mp: 97,
        def: [100, 380],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015600, 10015710, 10015300, 10049000, 10015707, 10013300, 0,
        ],
    },
    Monster {
        name: "Maze Mini Metallic",
        id: 10001005,
        monster_type: "PLANT_SKILL",
        level: 40,
        scale: 0.8,
        exp: 348,
        job: 33,
        hp: 10,
        sp: 58,
        mp: 97,
        def: [97, 380],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015600, 10015710, 10015300, 10049000, 10015707, 10013300, 0,
        ],
    },
    Monster {
        name: "Sakura Pururu",
        id: 10005000,
        monster_type: "PLANT_MARK",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 1,
        sp: 0,
        mp: 0,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10051800, 10051800, 10005645, 10025959, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sakura Pururu",
        id: 10005001,
        monster_type: "PLANT_MARK",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 2,
        sp: 0,
        mp: 0,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10051800, 10051800, 10005645, 10025959, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sakura Pururu",
        id: 10005002,
        monster_type: "PLANT_MARK",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 3,
        sp: 0,
        mp: 0,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "ChockKo",
        id: 10010000,
        monster_type: "BIRD",
        level: 8,
        scale: 1.0,
        exp: 54,
        job: 51,
        hp: 50,
        sp: 33,
        mp: 19,
        def: [6, 3],
        mdef: [10, 13],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007700, 10018208, 90000044, 0, /*null*/
            10019900, 90000044, 10050913,
        ],
    },
    Monster {
        name: "ChockKo(3",
        id: 10010400,
        monster_type: "BIRD",
        level: 8,
        scale: 1.0,
        exp: 54,
        job: 51,
        hp: 50,
        sp: 33,
        mp: 19,
        def: [6, 3],
        mdef: [10, 13],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007700, 10018208, 90000044, 0, /*null*/
            10019900, 90000044, 10050913,
        ],
    },
    Monster {
        name: "ChockKo(2 ",
        id: 10010800,
        monster_type: "BIRD",
        level: 8,
        scale: 1.0,
        exp: 54,
        job: 51,
        hp: 50,
        sp: 33,
        mp: 19,
        def: [6, 3],
        mdef: [10, 13],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007700, 10018208, 90000044, 0, /*null*/
            10019900, 90000044, 10050913,
        ],
    },
    Monster {
        name: "ChockKo(1",
        id: 10011400,
        monster_type: "BIRD",
        level: 8,
        scale: 1.0,
        exp: 54,
        job: 51,
        hp: 50,
        sp: 33,
        mp: 19,
        def: [6, 3],
        mdef: [10, 13],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007700, 10018208, 90000044, 0, /*null*/
            10019900, 90000044, 10050913,
        ],
    },
    Monster {
        name: "Infinite ChockKo",
        id: 10010002,
        monster_type: "BIRD",
        level: 46,
        scale: 1.0,
        exp: 54,
        job: 51,
        hp: 875,
        sp: 33,
        mp: 19,
        def: [20, 200],
        mdef: [20, 130],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Phoenix",
        id: 10010004,
        monster_type: "BIRD",
        level: 8,
        scale: 1.0,
        exp: 54,
        job: 51,
        hp: 50,
        sp: 33,
        mp: 19,
        def: [6, 3],
        mdef: [10, 13],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007700, 10018208, 90000044, 0, /*null*/
            10019900, 90000044, 0,
        ],
    },
    Monster {
        name: "Young CockKo",
        id: 10010101,
        monster_type: "BIRD",
        level: 25,
        scale: 1.0,
        exp: 348,
        job: 312,
        hp: 450,
        sp: 100,
        mp: 100,
        def: [12, 48],
        mdef: [35, 65],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 25,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Cockatrice",
        id: 10010102,
        monster_type: "BIRD",
        level: 55,
        scale: 1.0,
        exp: 306,
        job: 99,
        hp: 1000,
        sp: 43,
        mp: 50,
        def: [32, 110],
        mdef: [15, 162],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Cockatrice",
        id: 10010100,
        monster_type: "BIRD",
        level: 19,
        scale: 1.0,
        exp: 306,
        job: 99,
        hp: 130,
        sp: 43,
        mp: 50,
        def: [11, 33],
        mdef: [15, 42],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018200, 10006400, 10007700, 10012550, 10012550, 10034150, 10050921,
        ],
    },
    Monster {
        name: "Fire Rooster",
        id: 10015000,
        monster_type: "BIRD",
        level: 40,
        scale: 1.0,
        exp: 2430,
        job: 2430,
        hp: 1560,
        sp: 150,
        mp: 150,
        def: [10, 80],
        mdef: [35, 145],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10018200, 10007800, 10034502, 10037300, 10009500, 0,
        ],
    },
    Monster {
        name: "Draki",
        id: 10020000,
        monster_type: "BIRD",
        level: 19,
        scale: 1.0,
        exp: 312,
        job: 141,
        hp: 230,
        sp: 62,
        mp: 112,
        def: [9, 9],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            10006300, 0, /*null*/
            90000044, 0, /*null*/
            10019900, 10045100, 10050941,
        ],
    },
    Monster {
        name: "Cut Bat",
        id: 10020001,
        monster_type: "BIRD",
        level: 28,
        scale: 1.0,
        exp: 585,
        job: 315,
        hp: 410,
        sp: 999,
        mp: 999,
        def: [10, 59],
        mdef: [21, 35],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 0, /*null*/
            10019900, 10049000, 10009500, 10045100, 10050961,
        ],
    },
    Monster {
        name: "Infinite Cut Bat",
        id: 10020002,
        monster_type: "BIRD",
        level: 21,
        scale: 1.0,
        exp: 54,
        job: 45,
        hp: 488,
        sp: 1000,
        mp: 1000,
        def: [10, 35],
        mdef: [10, 62],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Crimson Bat",
        id: 10020100,
        monster_type: "BIRD",
        level: 23,
        scale: 1.0,
        exp: 576,
        job: 1485,
        hp: 650,
        sp: 150,
        mp: 150,
        def: [15, 45],
        mdef: [10, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10015350, 10019900, 0, /*null*/
            90000046, 10034150, 90000046, 0,
        ],
    },
    Monster {
        name: "Crimson Bat",
        id: 10020101,
        monster_type: "BIRD",
        level: 1,
        scale: 1.0,
        exp: 576,
        job: 1485,
        hp: 650,
        sp: 150,
        mp: 150,
        def: [15, 45],
        mdef: [10, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mars",
        id: 10020102,
        monster_type: "BIRD",
        level: 9,
        scale: 1.0,
        exp: 108,
        job: 9,
        hp: 110,
        sp: 0,
        mp: 0,
        def: [7, 15],
        mdef: [10, 4],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031909, 0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Necro Bat",
        id: 10020200,
        monster_type: "BIRD",
        level: 43,
        scale: 1.0,
        exp: 2025,
        job: 2475,
        hp: 1500,
        sp: 150,
        mp: 150,
        def: [30, 40],
        mdef: [20, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            10006300, 0, /*null*/
            90000044, 0, /*null*/
            10019900, 10045100, 0,
        ],
    },
    Monster {
        name: "Heavy Bat",
        id: 10020700,
        monster_type: "BIRD",
        level: 65,
        scale: 1.0,
        exp: 7425,
        job: 5625,
        hp: 2800,
        sp: 200,
        mp: 200,
        def: [10, 170],
        mdef: [10, 160],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10025207, 10019900, 10014852, 10054600, 10054600, 0,
        ],
    },
    Monster {
        name: "Venus",
        id: 10020701,
        monster_type: "BIRD",
        level: 24,
        scale: 1.0,
        exp: 411,
        job: 207,
        hp: 380,
        sp: 0,
        mp: 0,
        def: [11, 39],
        mdef: [14, 56],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031909, 0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Night Bat",
        id: 10020900,
        monster_type: "BIRD_UNITE",
        level: 25,
        scale: 1.5,
        exp: 402,
        job: 540,
        hp: 430,
        sp: 100,
        mp: 112,
        def: [12, 45],
        mdef: [10, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            10006300, 0, /*null*/
            90000044, 0, /*null*/
            10019900, 10045100, 0,
        ],
    },
    Monster {
        name: "Pluto",
        id: 10020901,
        monster_type: "BIRD",
        level: 43,
        scale: 1.0,
        exp: 2859,
        job: 1500,
        hp: 1200,
        sp: 0,
        mp: 0,
        def: [15, 150],
        mdef: [24, 76],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10031909, 0, /*null*/
            0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Steel Bat",
        id: 10021000,
        monster_type: "BIRD",
        level: 19,
        scale: 1.5,
        exp: 312,
        job: 141,
        hp: 230,
        sp: 62,
        mp: 112,
        def: [9, 9],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            10006300, 0, /*null*/
            90000044, 0, /*null*/
            10019900, 10045100, 0,
        ],
    },
    Monster {
        name: "Haumea",
        id: 10021001,
        monster_type: "INSECT",
        level: 77,
        scale: 1.0,
        exp: 11250,
        job: 10800,
        hp: 5000,
        sp: 77,
        mp: 0,
        def: [30, 151],
        mdef: [80, 200],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 10,
        },
        drop_ids: [
            10031909, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Draki (New Colors)",
        id: 10021400,
        monster_type: "BIRD",
        level: 19,
        scale: 1.0,
        exp: 312,
        job: 141,
        hp: 230,
        sp: 62,
        mp: 112,
        def: [9, 9],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            10006300, 0, /*null*/
            90000044, 0, /*null*/
            10019900, 10045100, 10050941,
        ],
    },
    Monster {
        name: "Bear",
        id: 10030000,
        monster_type: "ANIMAL",
        level: 22,
        scale: 1.0,
        exp: 339,
        job: 204,
        hp: 220,
        sp: 100,
        mp: 150,
        def: [25, 40],
        mdef: [22, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002800, 10020500, 10019900, 10020550, 10020500, 10020550, 10050919,
        ],
    },
    Monster {
        name: "Cave Bear",
        id: 10030100,
        monster_type: "ANIMAL",
        level: 24,
        scale: 1.0,
        exp: 405,
        job: 315,
        hp: 290,
        sp: 130,
        mp: 300,
        def: [32, 53],
        mdef: [40, 29],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020300, 10020500, 10019900, 10020550, 10020550, 10020201, 0,
        ],
    },
    Monster {
        name: "Red Beast",
        id: 10030101,
        monster_type: "ANIMAL",
        level: 37,
        scale: 1.5,
        exp: 1620,
        job: 1080,
        hp: 780,
        sp: 999,
        mp: 999,
        def: [25, 60],
        mdef: [60, 15],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020300, 10020500, 10019900, 10020201, 10020201, 10020400, 0,
        ],
    },
    Monster {
        name: "Infinite Red Beast",
        id: 10030102,
        monster_type: "ANIMAL",
        level: 16,
        scale: 1.0,
        exp: 36,
        job: 18,
        hp: 360,
        sp: 1000,
        mp: 1000,
        def: [10, 31],
        mdef: [10, 32],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Red Beast",
        id: 10030103,
        monster_type: "ANIMAL",
        level: 37,
        scale: 1.0,
        exp: 270,
        job: 270,
        hp: 1150,
        sp: 1000,
        mp: 1000,
        def: [10, 61],
        mdef: [10, 49],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Polar Bear",
        id: 10030400,
        monster_type: "ANIMAL",
        level: 22,
        scale: 1.0,
        exp: 294,
        job: 249,
        hp: 340,
        sp: 105,
        mp: 250,
        def: [10, 60],
        mdef: [45, 24],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009000, 10020500, 10019900, 10020551, 10020500, 10020551, 10050926,
        ],
    },
    Monster {
        name: "King Polar Bear",
        id: 10030401,
        monster_type: "ANIMAL",
        level: 38,
        scale: 1.5,
        exp: 1800,
        job: 900,
        hp: 800,
        sp: 105,
        mp: 250,
        def: [33, 76],
        mdef: [30, 15],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020500, 10019808, 10019900, 10020551, 10020551, 10020550, 0,
        ],
    },
    Monster {
        name: "Infinite King Polar Bear",
        id: 10030402,
        monster_type: "ANIMAL",
        level: 36,
        scale: 1.0,
        exp: 225,
        job: 225,
        hp: 1150,
        sp: 1000,
        mp: 1000,
        def: [10, 59],
        mdef: [10, 53],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Violent Beast",
        id: 10030403,
        monster_type: "ANIMAL",
        level: 51,
        scale: 3.0,
        exp: 4500,
        job: 4500,
        hp: 2000,
        sp: 0,
        mp: 0,
        def: [52, 110],
        mdef: [63, 215],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020500, 10019808, 10019900, 10020551, 10020551, 10020550, 0,
        ],
    },
    Monster {
        name: "Green Beast",
        id: 10030500,
        monster_type: "ANIMAL",
        level: 0,
        scale: 1.0,
        exp: 360,
        job: 315,
        hp: 500,
        sp: 156,
        mp: 451,
        def: [50, 52],
        mdef: [50, 12],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10019900, 0, /*null*/
            10009500, 10009600, 0,
        ],
    },
    Monster {
        name: "Killer Beast",
        id: 10030700,
        monster_type: "ANIMAL",
        level: 37,
        scale: 1.0,
        exp: 1350,
        job: 1350,
        hp: 550,
        sp: 999,
        mp: 999,
        def: [15, 100],
        mdef: [20, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020300, 10020500, 10019900, 10020400, 10020400, 10020409, 10050967,
        ],
    },
    Monster {
        name: "Black Bear",
        id: 10030900,
        monster_type: "ANIMAL",
        level: 41,
        scale: 1.0,
        exp: 2610,
        job: 1170,
        hp: 650,
        sp: 126,
        mp: 342,
        def: [18, 122],
        mdef: [55, 86],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            10020300, 10020500, 10019900, 10020409, 10020409, 10020309, 10050969,
        ],
    },
    Monster {
        name: "Most Powerful Evil",
        id: 10030901,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 43,
        scale: 2.0,
        exp: 193950,
        job: 58050,
        hp: 68000,
        sp: 126,
        mp: 342,
        def: [55, 165],
        mdef: [35, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020500, 10020310, 10020409, 10020350, 10049500, 10020350, 10049500, 10051002,
        ],
    },
    Monster {
        name: "Infinite Black Bear",
        id: 10030902,
        monster_type: "ANIMAL",
        level: 23,
        scale: 1.0,
        exp: 81,
        job: 36,
        hp: 180,
        sp: 1000,
        mp: 1000,
        def: [10, 22],
        mdef: [25, 24],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Dark Bear",
        id: 10030903,
        monster_type: "ANIMAL",
        level: 39,
        scale: 1.2,
        exp: 10890,
        job: 1485,
        hp: 2000,
        sp: 126,
        mp: 342,
        def: [20, 90],
        mdef: [30, 160],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10020300, 10020500, 10067300, 10020409, 10020409, 10020309, 0,
        ],
    },
    Monster {
        name: "Most Powerful Evil",
        id: 10030904,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 43,
        scale: 2.0,
        exp: 193950,
        job: 58050,
        hp: 68000,
        sp: 126,
        mp: 342,
        def: [55, 165],
        mdef: [35, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10051002,
        ],
    },
    Monster {
        name: "Killer Bee",
        id: 10040000,
        monster_type: "INSECT",
        level: 7,
        scale: 1.0,
        exp: 51,
        job: 42,
        hp: 50,
        sp: 40,
        mp: 0,
        def: [4, 6],
        mdef: [21, 35],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10035400, 10035200, 90000046, 10034507, 10034507, 90000046, 10050912,
        ],
    },
    Monster {
        name: "Infinite Killer Bee",
        id: 10040002,
        monster_type: "INSECT_BOSS",
        level: 30,
        scale: 2.0,
        exp: 99,
        job: 90,
        hp: 1350,
        sp: 1000,
        mp: 1000,
        def: [10, 55],
        mdef: [10, 49],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Working Bee",
        id: 10040003,
        monster_type: "INSECT",
        level: 4,
        scale: 0.5,
        exp: 27,
        job: 24,
        hp: 88,
        sp: 40,
        mp: 0,
        def: [5, 7],
        mdef: [25, 39],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10003200, 10067300, 10003200, 10003200, 10003200, 0,
        ],
    },
    Monster {
        name: "Assassin Bee",
        id: 10040004,
        monster_type: "INSECT",
        level: 7,
        scale: 1.0,
        exp: 51,
        job: 42,
        hp: 50,
        sp: 40,
        mp: 0,
        def: [4, 6],
        mdef: [21, 35],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10035400, 10035200, 90000046, 10034507, 10034507, 90000046, 10050912,
        ],
    },
    Monster {
        name: "Wasp",
        id: 10040100,
        monster_type: "INSECT",
        level: 20,
        scale: 1.0,
        exp: 231,
        job: 222,
        hp: 155,
        sp: 60,
        mp: 0,
        def: [9, 13],
        mdef: [10, 5],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10035200, 10000408, 90000046, 0, /*null*/
            10000408, 10009500, 10050932,
        ],
    },
    Monster {
        name: "Infinite Wasp",
        id: 10040101,
        monster_type: "INSECT",
        level: 24,
        scale: 1.0,
        exp: 81,
        job: 45,
        hp: 650,
        sp: 1000,
        mp: 1000,
        def: [10, 57],
        mdef: [10, 63],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Queen Bee",
        id: 10040200,
        monster_type: "INSECT",
        level: 32,
        scale: 1.0,
        exp: 789,
        job: 789,
        hp: 245,
        sp: 55,
        mp: 0,
        def: [15, 51],
        mdef: [40, 76],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10035400, 10000408, 90000046, 0, /*null*/
            10000408, 10009500, 10050936,
        ],
    },
    Monster {
        name: "Infinite Queen Bee",
        id: 10040201,
        monster_type: "INSECT",
        level: 42,
        scale: 1.0,
        exp: 339,
        job: 339,
        hp: 545,
        sp: 55,
        mp: 0,
        def: [25, 61],
        mdef: [48, 86],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Shining Bee",
        id: 10040300,
        monster_type: "INSECT",
        level: 90,
        scale: 1.0,
        exp: 19350,
        job: 18000,
        hp: 10000,
        sp: 40,
        mp: 0,
        def: [80, 300],
        mdef: [50, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 10,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009000, 10002807, 10033350, 10019900, 10033350, 90000045, 0,
        ],
    },
    Monster {
        name: "Shining Bee",
        id: 10040900,
        monster_type: "INSECT",
        level: 75,
        scale: 1.0,
        exp: 10575,
        job: 11250,
        hp: 4600,
        sp: 40,
        mp: 0,
        def: [100, 155],
        mdef: [25, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10009000, 10016150, 10035400, 10019900, 10016150, 90000045, 0,
        ],
    },
    Monster {
        name: "Queen Wasp",
        id: 10041500,
        monster_type: "INSECT",
        level: 23,
        scale: 1.0,
        exp: 294,
        job: 384,
        hp: 450,
        sp: 40,
        mp: 0,
        def: [15, 55],
        mdef: [21, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10035400, 10035200, 90000046, 10034507, 10034507, 90000046, 0,
        ],
    },
    Monster {
        name: "Venomous Stinger",
        id: 10041501,
        monster_type: "INSECT_BOSS_SKILL",
        level: 60,
        scale: 3.5,
        exp: 45000,
        job: 63000,
        hp: 150000,
        sp: 40,
        mp: 0,
        def: [10, 160],
        mdef: [35, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 50,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            10000408, 10000408, 10000408, 10000408, 10011600, 10000408, 10009111, 0,
        ],
    },
    Monster {
        name: "Crawler",
        id: 10050000,
        monster_type: "INSECT",
        level: 3,
        scale: 1.2,
        exp: 24,
        job: 15,
        hp: 60,
        sp: 32,
        mp: 20,
        def: [3, 2],
        mdef: [8, 2],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007500, 10001903, 10032805, 0, /*null*/
            10034800, 10034850, 10050911,
        ],
    },
    Monster {
        name: "Giant Crawler",
        id: 10050001,
        monster_type: "INSECT",
        level: 15,
        scale: 2.0,
        exp: 450,
        job: 270,
        hp: 3000,
        sp: 32,
        mp: 20,
        def: [4, 21],
        mdef: [8, 3],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007500, 10001903, 10032805, 10049000, 10034800, 10034850, 10050918,
        ],
    },
    Monster {
        name: "Dimension Crawler",
        id: 10050002,
        monster_type: "INSECT",
        level: 3,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 32,
        mp: 20,
        def: [3, 2],
        mdef: [8, 2],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Caterpillar",
        id: 10050100,
        monster_type: "INSECT",
        level: 16,
        scale: 1.2,
        exp: 159,
        job: 114,
        hp: 160,
        sp: 47,
        mp: 76,
        def: [11, 29],
        mdef: [15, 39],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007500, 0, /*null*/
            10032801, 0, /*null*/
            10034800, 10034850, 10050916,
        ],
    },
    Monster {
        name: "Infinite Caterpillar",
        id: 10050101,
        monster_type: "INSECT",
        level: 8,
        scale: 1.0,
        exp: 18,
        job: 9,
        hp: 140,
        sp: 1000,
        mp: 1000,
        def: [15, 30],
        mdef: [35, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Rock Eater",
        id: 10050102,
        monster_type: "INSECT",
        level: 40,
        scale: 1.3,
        exp: 1824,
        job: 1689,
        hp: 930,
        sp: 100,
        mp: 100,
        def: [32, 84],
        mdef: [48, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014650, 10014853, 90000046, 10017000, 10014800, 10014851, 10050985,
        ],
    },
    Monster {
        name: "Infinite Rock Eater",
        id: 10050103,
        monster_type: "INSECT",
        level: 47,
        scale: 1.3,
        exp: 1374,
        job: 675,
        hp: 1420,
        sp: 100,
        mp: 100,
        def: [10, 150],
        mdef: [20, 105],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Elder Worm",
        id: 10050200,
        monster_type: "INSECT",
        level: 20,
        scale: 1.2,
        exp: 225,
        job: 225,
        hp: 160,
        sp: 55,
        mp: 117,
        def: [21, 17],
        mdef: [15, 25],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007500, 10009350, 10032802, 10049000, 10034800, 10034850, 10050922,
        ],
    },
    Monster {
        name: "Giant Elder Worm",
        id: 10050201,
        monster_type: "INSECT",
        level: 50,
        scale: 3.0,
        exp: 225,
        job: 225,
        hp: 500000,
        sp: 89,
        mp: 203,
        def: [1, 200],
        mdef: [65, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10049000, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ralba",
        id: 10050400,
        monster_type: "INSECT",
        level: 0,
        scale: 1.0,
        exp: 6,
        job: 0,
        hp: 100,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sand Crawler",
        id: 10050800,
        monster_type: "INSECT",
        level: 14,
        scale: 1.2,
        exp: 114,
        job: 69,
        hp: 135,
        sp: 43,
        mp: 59,
        def: [14, 24],
        mdef: [14, 25],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007500, 10009350, 10032807, 0, /*null*/
            10034800, 10034850, 10050930,
        ],
    },
    Monster {
        name: "Infinite Crawler",
        id: 10050801,
        monster_type: "INSECT_BOSS",
        level: 23,
        scale: 1.5,
        exp: 72,
        job: 45,
        hp: 700,
        sp: 1000,
        mp: 1000,
        def: [20, 35],
        mdef: [40, 27],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 10,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Fluff Worm",
        id: 10055000,
        monster_type: "INSECT",
        level: 26,
        scale: 1.2,
        exp: 450,
        job: 324,
        hp: 385,
        sp: 32,
        mp: 20,
        def: [4, 21],
        mdef: [16, 12],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007500, 10033905, 10020500, 0, /*null*/
            10020551, 10007581, 0,
        ],
    },
    Monster {
        name: "Fly Fish",
        id: 10060000,
        monster_type: "WATER_ANIMAL",
        level: 20,
        scale: 1.0,
        exp: 243,
        job: 207,
        hp: 160,
        sp: 44,
        mp: 49,
        def: [12, 28],
        mdef: [8, 33],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037500, 10007400, 10007250, 10007450, 10007400, 10003950, 10050923,
        ],
    },
    Monster {
        name: "Farm Fly Fish",
        id: 10060001,
        monster_type: "WATER_ANIMAL",
        level: 17,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 44,
        mp: 49,
        def: [100, 120],
        mdef: [100, 100],
        properties: Properties {
            fire: 0,
            water: 10,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Cave Fish",
        id: 10060002,
        monster_type: "WATER_ANIMAL",
        level: 27,
        scale: 1.0,
        exp: 405,
        job: 315,
        hp: 370,
        sp: 999,
        mp: 999,
        def: [15, 31],
        mdef: [22, 64],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037500, 10007400, 10007250, 10007450, 10007400, 10003950, 10050946,
        ],
    },
    Monster {
        name: "Infinite Cave Fish",
        id: 10060003,
        monster_type: "WATER_ANIMAL",
        level: 18,
        scale: 1.0,
        exp: 45,
        job: 27,
        hp: 280,
        sp: 1000,
        mp: 1000,
        def: [10, 36],
        mdef: [10, 46],
        properties: Properties {
            fire: 0,
            water: 10,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Tropical Fish",
        id: 10060004,
        monster_type: "WATER_ANIMAL",
        level: 37,
        scale: 1.0,
        exp: 1917,
        job: 735,
        hp: 728,
        sp: 100,
        mp: 100,
        def: [12, 35],
        mdef: [32, 53],
        properties: Properties {
            fire: 0,
            water: 40,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037500, 10007400, 10007250, 10049000, 10007400, 10003950, 10050983,
        ],
    },
    Monster {
        name: "Infinite Fly Fish",
        id: 10060005,
        monster_type: "WATER_ANIMAL",
        level: 53,
        scale: 1.0,
        exp: 1440,
        job: 1080,
        hp: 1640,
        sp: 44,
        mp: 49,
        def: [20, 150],
        mdef: [20, 89],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 25,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fish",
        id: 10060006,
        monster_type: "WATER_ANIMAL_NOTOUCH",
        level: 25,
        scale: 0.3,
        exp: 0,
        job: 0,
        hp: 760,
        sp: 100,
        mp: 100,
        def: [12, 55],
        mdef: [21, 49],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 22,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Deep Sea Fish",
        id: 10060200,
        monster_type: "WATER_ANIMAL",
        level: 75,
        scale: 0.8,
        exp: 11250,
        job: 11700,
        hp: 11750,
        sp: 400,
        mp: 500,
        def: [20, 250],
        mdef: [70, 130],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018204, 10014501, 90000046, 0, /*null*/
            10007250, 10007450, 0,
        ],
    },
    Monster {
        name: "Deep Sea Fish",
        id: 10060201,
        monster_type: "WATER_ANIMAL_BOSS_SKILL",
        level: 75,
        scale: 1.2,
        exp: 11250,
        job: 11700,
        hp: 11750,
        sp: 400,
        mp: 500,
        def: [20, 250],
        mdef: [70, 130],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018204, 10014501, 90000046, 0, /*null*/
            10007250, 10007450, 0,
        ],
    },
    Monster {
        name: "Angel Fish",
        id: 10060600,
        monster_type: "WATER_ANIMAL",
        level: 87,
        scale: 0.8,
        exp: 19800,
        job: 20925,
        hp: 19850,
        sp: 500,
        mp: 600,
        def: [50, 170],
        mdef: [50, 80],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10062000, 10018207, 10007450, 0, /*null*/
            10062000, 10007450, 0,
        ],
    },
    Monster {
        name: "Angel Fly Fish",
        id: 10060601,
        monster_type: "WATER_ANIMAL_BOSS_SKILL",
        level: 87,
        scale: 1.2,
        exp: 19800,
        job: 20925,
        hp: 19850,
        sp: 500,
        mp: 600,
        def: [50, 170],
        mdef: [30, 80],
        properties: Properties {
            fire: 0,
            water: 5,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10062000, 10018207, 10007450, 0, /*null*/
            10062000, 10007450, 0,
        ],
    },
    Monster {
        name: "Death Fish",
        id: 10061000,
        monster_type: "WATER_ANIMAL",
        level: 15,
        scale: 1.0,
        exp: 225,
        job: 225,
        hp: 150,
        sp: 44,
        mp: 49,
        def: [70, 50],
        mdef: [50, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037500, 10007400, 10007250, 10007450, 10007400, 10003950, 0,
        ],
    },
    Monster {
        name: "Urchin",
        id: 10070000,
        monster_type: "ANIMAL",
        level: 2,
        scale: 1.0,
        exp: 18,
        job: 9,
        hp: 45,
        sp: 31,
        mp: 12,
        def: [2, 1],
        mdef: [6, 2],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025200, 10000104, 10000701, 10034507, 10003750, 10034508, 10050910,
        ],
    },
    Monster {
        name: "Infinite Kangkang",
        id: 10070100,
        monster_type: "ROCK",
        level: 13,
        scale: 1.0,
        exp: 27,
        job: 18,
        hp: 200,
        sp: 1000,
        mp: 1000,
        def: [10, 22],
        mdef: [15, 29],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Torpedo",
        id: 10070200,
        monster_type: "ANIMAL",
        level: 29,
        scale: 1.0,
        exp: 744,
        job: 384,
        hp: 330,
        sp: 67,
        mp: 126,
        def: [10, 48],
        mdef: [15, 64],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10001550, 10019900, 0, /*null*/
            10009500, 10009600, 0,
        ],
    },
    Monster {
        name: "Crash Ice",
        id: 10070300,
        monster_type: "ANIMAL",
        level: 20,
        scale: 1.0,
        exp: 252,
        job: 198,
        hp: 160,
        sp: 44,
        mp: 49,
        def: [10, 48],
        mdef: [15, 5],
        properties: Properties {
            fire: 0,
            water: 10,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000210, 10001800, 10001804, 10049000, 10001907, 10019350, 10050924,
        ],
    },
    Monster {
        name: "Edge",
        id: 10070400,
        monster_type: "ANIMAL",
        level: 24,
        scale: 1.0,
        exp: 294,
        job: 294,
        hp: 600,
        sp: 50,
        mp: 65,
        def: [16, 61],
        mdef: [18, 50],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002200, 10008450, 10025204, 10049000, 10037450, 10009300, 10050950,
        ],
    },
    Monster {
        name: "Salad Urchin",
        id: 10070600,
        monster_type: "ANIMAL",
        level: 13,
        scale: 1.0,
        exp: 90,
        job: 69,
        hp: 100,
        sp: 47,
        mp: 56,
        def: [10, 16],
        mdef: [8, 17],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006850, 10006800, 10032806, 10049000, 10006800, 10001905, 10050915,
        ],
    },
    Monster {
        name: "Infinite Salad Urchin",
        id: 10070601,
        monster_type: "ANIMAL",
        level: 36,
        scale: 1.0,
        exp: 270,
        job: 180,
        hp: 750,
        sp: 1000,
        mp: 1000,
        def: [10, 66],
        mdef: [10, 72],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Morningstar",
        id: 10070700,
        monster_type: "ANIMAL",
        level: 32,
        scale: 1.0,
        exp: 1284,
        job: 744,
        hp: 540,
        sp: 999,
        mp: 999,
        def: [66, 44],
        mdef: [28, 61],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015600, 10015700, 10015710, 0, /*null*/
            10015000, 10015800, 10050963,
        ],
    },
    Monster {
        name: "Tanker",
        id: 10070800,
        monster_type: "ANIMAL",
        level: 18,
        scale: 1.0,
        exp: 213,
        job: 150,
        hp: 111,
        sp: 37,
        mp: 23,
        def: [12, 18],
        mdef: [11, 51],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10022309, 10007600, 10000701, 0, /*null*/
            10035300, 10000701, 10050931,
        ],
    },
    Monster {
        name: "Kangkang",
        id: 10070801,
        monster_type: "ROCK",
        level: 36,
        scale: 1.0,
        exp: 1215,
        job: 693,
        hp: 600,
        sp: 100,
        mp: 100,
        def: [25, 125],
        mdef: [39, 135],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10001550, 10003750, 10049000, 10014650, 10015400, 10050982,
        ],
    },
    Monster {
        name: "Caviar Urchin",
        id: 10070900,
        monster_type: "ANIMAL",
        level: 15,
        scale: 1.0,
        exp: 114,
        job: 114,
        hp: 90,
        sp: 38,
        mp: 31,
        def: [8, 14],
        mdef: [12, 32],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10003750, 10015250, 10026350, 0, /*null*/
            10009500, 10009600, 0,
        ],
    },
    Monster {
        name: "Metal Urchin",
        id: 10071000,
        monster_type: "ANIMAL",
        level: 30,
        scale: 1.0,
        exp: 720,
        job: 630,
        hp: 250,
        sp: 73,
        mp: 159,
        def: [80, 40],
        mdef: [40, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10017000, 10015600, 10025210, 0, /*null*/
            10015707, 10013300, 10050964,
        ],
    },
    Monster {
        name: "Roper",
        id: 10080000,
        monster_type: "PLANT",
        level: 21,
        scale: 0.8,
        exp: 360,
        job: 339,
        hp: 120,
        sp: 55,
        mp: 89,
        def: [50, 1],
        mdef: [50, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10025100, 90000045, 0, /*null*/
            10009100, 10009500, 10050933,
        ],
    },
    Monster {
        name: "Infinite Roper",
        id: 10080001,
        monster_type: "PLANT",
        level: 12,
        scale: 1.0,
        exp: 24,
        job: 15,
        hp: 96,
        sp: 1000,
        mp: 1000,
        def: [10, 37],
        mdef: [10, 59],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "PuruPuru",
        id: 10080002,
        monster_type: "PLANT",
        level: 40,
        scale: 0.8,
        exp: 1512,
        job: 1404,
        hp: 660,
        sp: 100,
        mp: 100,
        def: [30, 68],
        mdef: [90, 88],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009100, 10025100, 10032808, 10049000, 10009500, 10009600, 10050986,
        ],
    },
    Monster {
        name: "Dentacle",
        id: 10080100,
        monster_type: "PLANT",
        level: 33,
        scale: 0.8,
        exp: 900,
        job: 1125,
        hp: 430,
        sp: 55,
        mp: 89,
        def: [20, 71],
        mdef: [90, 57],
        properties: Properties {
            fire: 0,
            water: 75,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009100, 10025100, 10019900, 10049000, 10009500, 10009600, 10050938,
        ],
    },
    Monster {
        name: "Infinite Dentacle",
        id: 10080101,
        monster_type: "PLANT_BOSS",
        level: 38,
        scale: 2.0,
        exp: 315,
        job: 315,
        hp: 2600,
        sp: 1000,
        mp: 1000,
        def: [10, 74],
        mdef: [10, 81],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mishandora",
        id: 10080900,
        monster_type: "PLANT",
        level: 21,
        scale: 0.8,
        exp: 360,
        job: 339,
        hp: 120,
        sp: 55,
        mp: 89,
        def: [50, 1],
        mdef: [50, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10025100, 90000045, 0, /*null*/
            10009100, 10009500, 10050933,
        ],
    },
    Monster {
        name: "Sea Anemone",
        id: 10081500,
        monster_type: "PLANT",
        level: 21,
        scale: 0.8,
        exp: 360,
        job: 339,
        hp: 120,
        sp: 55,
        mp: 89,
        def: [50, 1],
        mdef: [50, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10025100, 90000045, 0, /*null*/
            10009100, 10009500, 10050933,
        ],
    },
    Monster {
        name: "Arcana Spade",
        id: 10100000,
        monster_type: "MAGIC_CREATURE",
        level: 40,
        scale: 1.2,
        exp: 2475,
        job: 675,
        hp: 1400,
        sp: 110,
        mp: 255,
        def: [15, 0],
        mdef: [45, 0],
        properties: Properties {
            fire: 0,
            water: 40,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020000, 10000103, 10034103, 0, /*null*/
            10029300, 10029400, 10050975,
        ],
    },
    Monster {
        name: "Arcana Heart",
        id: 10100100,
        monster_type: "MAGIC_CREATURE",
        level: 40,
        scale: 1.2,
        exp: 2025,
        job: 1125,
        hp: 1000,
        sp: 110,
        mp: 255,
        def: [5, 0],
        mdef: [75, 0],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020000, 10000103, 10000604, 10049000, 10029300, 10029400, 10050974,
        ],
    },
    Monster {
        name: "Arcana Heart",
        id: 10100101,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 38,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 420,
        sp: 110,
        mp: 255,
        def: [5, 120],
        mdef: [75, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Arcana Diamond",
        id: 10100200,
        monster_type: "MAGIC_CREATURE",
        level: 40,
        scale: 1.2,
        exp: 1710,
        job: 1440,
        hp: 1600,
        sp: 110,
        mp: 255,
        def: [70, 0],
        mdef: [74, 0],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020000, 10000108, 10000604, 10049000, 10029300, 10029400, 10050973,
        ],
    },
    Monster {
        name: "Arcana Club",
        id: 10100300,
        monster_type: "MAGIC_CREATURE",
        level: 34,
        scale: 1.2,
        exp: 1350,
        job: 900,
        hp: 1234,
        sp: 110,
        mp: 255,
        def: [3, 25],
        mdef: [20, 48],
        properties: Properties {
            fire: 0,
            water: 40,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020000, 0, /*null*/
            10019900, 10049000, 10029300, 10029400, 10050971,
        ],
    },
    Monster {
        name: "Infinite Arcana Fighter",
        id: 10100301,
        monster_type: "MAGIC_CREATURE",
        level: 17,
        scale: 1.0,
        exp: 45,
        job: 18,
        hp: 116,
        sp: 1000,
        mp: 1000,
        def: [20, 19],
        mdef: [20, 14],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010000, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Fighter",
        id: 10100302,
        monster_type: "MAGIC_CREATURE",
        level: 17,
        scale: 1.0,
        exp: 45,
        job: 18,
        hp: 116,
        sp: 1000,
        mp: 1000,
        def: [20, 19],
        mdef: [20, 14],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010001, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Fighter",
        id: 10100303,
        monster_type: "MAGIC_CREATURE",
        level: 17,
        scale: 1.0,
        exp: 45,
        job: 18,
        hp: 116,
        sp: 1000,
        mp: 1000,
        def: [20, 19],
        mdef: [20, 14],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010002, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Fighter",
        id: 10100304,
        monster_type: "MAGIC_CREATURE",
        level: 17,
        scale: 1.0,
        exp: 45,
        job: 18,
        hp: 116,
        sp: 1000,
        mp: 1000,
        def: [20, 19],
        mdef: [20, 14],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010003, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Fighter",
        id: 10100305,
        monster_type: "MAGIC_CREATURE",
        level: 17,
        scale: 1.0,
        exp: 45,
        job: 18,
        hp: 116,
        sp: 1000,
        mp: 1000,
        def: [20, 19],
        mdef: [20, 14],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010004, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Willydoo",
        id: 10110000,
        monster_type: "ANIMAL",
        level: 31,
        scale: 1.0,
        exp: 1026,
        job: 942,
        hp: 300,
        sp: 60,
        mp: 255,
        def: [23, 45],
        mdef: [27, 73],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10009100, 10025300, 10049000, 10009100, 10009500, 10050951,
        ],
    },
    Monster {
        name: "Infinite Willydoo",
        id: 10110001,
        monster_type: "ANIMAL",
        level: 48,
        scale: 1.0,
        exp: 855,
        job: 495,
        hp: 1000,
        sp: 60,
        mp: 255,
        def: [10, 100],
        mdef: [20, 97],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 25,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Cherrydoo",
        id: 10110100,
        monster_type: "ANIMAL",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [50, 1],
        mdef: [50, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10051301, 10051301, 10051301, 10051301, 10051301, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Minidoo",
        id: 10110200,
        monster_type: "ANIMAL",
        level: 32,
        scale: 1.0,
        exp: 1059,
        job: 744,
        hp: 320,
        sp: 80,
        mp: 255,
        def: [20, 32],
        mdef: [50, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009100, 10009100, 10004200, 10017900, 10000102, 10009500, 10050970,
        ],
    },
    Monster {
        name: "Minidoo",
        id: 10110201,
        monster_type: "ANIMAL_BOMB_SKILL",
        level: 32,
        scale: 1.0,
        exp: 1059,
        job: 744,
        hp: 150000,
        sp: 80,
        mp: 255,
        def: [30, 0],
        mdef: [20, 0],
        properties: Properties {
            fire: 100,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10050970,
        ],
    },
    Monster {
        name: "Raichi",
        id: 10110400,
        monster_type: "ANIMAL",
        level: 78,
        scale: 1.0,
        exp: 10575,
        job: 11925,
        hp: 5000,
        sp: 255,
        mp: 255,
        def: [10, 250],
        mdef: [45, 73],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009000, 10009101, 10019900, 10049000, 10025350, 50031350, 0,
        ],
    },
    Monster {
        name: "Coconut",
        id: 10111000,
        monster_type: "ANIMAL",
        level: 60,
        scale: 1.0,
        exp: 4680,
        job: 3285,
        hp: 2100,
        sp: 155,
        mp: 155,
        def: [15, 140],
        mdef: [30, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009000, 10009101, 10019900, 10049000, 10009500, 10009600, 0,
        ],
    },
    Monster {
        name: "Cinnamon",
        id: 10111300,
        monster_type: "ANIMAL",
        level: 39,
        scale: 1.0,
        exp: 1386,
        job: 1329,
        hp: 525,
        sp: 100,
        mp: 100,
        def: [29, 50],
        mdef: [24, 52],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10034000, 10009100, 10004200, 10049000, 10000102, 10009500, 10050984,
        ],
    },
    Monster {
        name: "Infinite Cinnamon",
        id: 10111301,
        monster_type: "ANIMAL",
        level: 55,
        scale: 1.0,
        exp: 486,
        job: 429,
        hp: 1500,
        sp: 100,
        mp: 100,
        def: [39, 200],
        mdef: [12, 46],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Counter Cinnamon",
        id: 10111302,
        monster_type: "ANIMAL_BOSS_NOTPTDROPRANGE",
        level: 60,
        scale: 8.0,
        exp: 4500,
        job: 4500,
        hp: 500000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            90000043, 90000044, 90000045, 90000046, 90000043, 90000043, 90000044, 0,
        ],
    },
    Monster {
        name: "Moon Cinnamon",
        id: 10111303,
        monster_type: "ANIMAL",
        level: 3,
        scale: 1.0,
        exp: 45,
        job: 45,
        hp: 41,
        sp: 100,
        mp: 100,
        def: [2, 1],
        mdef: [6, 2],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10055404, 10055402, 10055403, 10055401, 10055405, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Moon Cinnamon",
        id: 10111304,
        monster_type: "ANIMAL",
        level: 3,
        scale: 0.8,
        exp: 45,
        job: 45,
        hp: 41,
        sp: 100,
        mp: 100,
        def: [2, 1],
        mdef: [6, 2],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10055402, 10055406, 10055405, 10055404, 10055403, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Bawoo",
        id: 10120000,
        monster_type: "ANIMAL",
        level: 9,
        scale: 1.0,
        exp: 108,
        job: 9,
        hp: 110,
        sp: 0,
        mp: 0,
        def: [7, 15],
        mdef: [10, 4],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006600, 10006300, 90000043, 10048903, 10020200, 10048903, 10050914,
        ],
    },
    Monster {
        name: "Infinite Bawoo",
        id: 10120001,
        monster_type: "ANIMAL",
        level: 12,
        scale: 1.0,
        exp: 27,
        job: 9,
        hp: 108,
        sp: 1000,
        mp: 1000,
        def: [10, 10],
        mdef: [10, 13],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Bawoo Bawoo",
        id: 10120002,
        monster_type: "ANIMAL",
        level: 25,
        scale: 1.0,
        exp: 360,
        job: 261,
        hp: 760,
        sp: 100,
        mp: 100,
        def: [12, 55],
        mdef: [21, 49],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 22,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Taro",
        id: 10120003,
        monster_type: "ANIMAL_NOTOUCH",
        level: 25,
        scale: 1.0,
        exp: 360,
        job: 261,
        hp: 760,
        sp: 100,
        mp: 100,
        def: [12, 55],
        mdef: [21, 49],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 22,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wild Bawoo",
        id: 10120004,
        monster_type: "ANIMAL_NOTOUCH",
        level: 25,
        scale: 1.0,
        exp: 360,
        job: 261,
        hp: 760,
        sp: 100,
        mp: 100,
        def: [12, 55],
        mdef: [21, 49],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 22,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nora",
        id: 10120005,
        monster_type: "ANIMAL_NOTOUCH",
        level: 25,
        scale: 1.0,
        exp: 360,
        job: 261,
        hp: 760,
        sp: 100,
        mp: 100,
        def: [12, 55],
        mdef: [21, 49],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 22,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Crimson Bawoo",
        id: 10120100,
        monster_type: "ANIMAL",
        level: 17,
        scale: 1.0,
        exp: 180,
        job: 135,
        hp: 150,
        sp: 0,
        mp: 0,
        def: [14, 32],
        mdef: [10, 10],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 50044500, 0, /*null*/
            10020210, 10006650, 10050920,
        ],
    },
    Monster {
        name: "Royal Blue Bawoo",
        id: 10120400,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 30,
        scale: 3.0,
        exp: 4725,
        job: 4275,
        hp: 5500,
        sp: 500,
        mp: 500,
        def: [25, 45],
        mdef: [15, 45],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10006300, 10020300, 10020300, 50030651, 50031500, 10006400, 10006650, 0,
        ],
    },
    Monster {
        name: "Royal Blue Bawoo",
        id: 10120401,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 30,
        scale: 3.0,
        exp: 4725,
        job: 4275,
        hp: 5500,
        sp: 500,
        mp: 500,
        def: [25, 45],
        mdef: [15, 45],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10006300, 10020300, 10020300, 50030651, 50031500, 10006400, 10006650, 0,
        ],
    },
    Monster {
        name: "Crimson Chief",
        id: 10120700,
        monster_type: "ANIMAL",
        level: 28,
        scale: 1.0,
        exp: 474,
        job: 429,
        hp: 420,
        sp: 999,
        mp: 999,
        def: [25, 45],
        mdef: [15, 45],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10020300, 10049000, 10020210, 10020400, 10050962,
        ],
    },
    Monster {
        name: "Yellow Bawoo",
        id: 10120800,
        monster_type: "ANIMAL",
        level: 9,
        scale: 1.0,
        exp: 108,
        job: 9,
        hp: 110,
        sp: 0,
        mp: 0,
        def: [7, 15],
        mdef: [10, 4],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006600, 10006300, 90000043, 0, /*null*/
            10020200, 10020210, 0,
        ],
    },
    Monster {
        name: "Grey Bawoo",
        id: 10121000,
        monster_type: "ANIMAL",
        level: 9,
        scale: 1.0,
        exp: 108,
        job: 9,
        hp: 110,
        sp: 0,
        mp: 0,
        def: [7, 15],
        mdef: [10, 4],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006600, 10006300, 90000043, 0, /*null*/
            10020200, 10020210, 0,
        ],
    },
    Monster {
        name: "Peach Bawoo",
        id: 10121500,
        monster_type: "ANIMAL",
        level: 9,
        scale: 1.0,
        exp: 108,
        job: 9,
        hp: 110,
        sp: 0,
        mp: 0,
        def: [7, 15],
        mdef: [10, 4],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006600, 10006300, 90000043, 0, /*null*/
            10020200, 10020210, 0,
        ],
    },
    Monster {
        name: "Clone Boochi Bawoo",
        id: 10122000,
        monster_type: "ANIMAL",
        level: 29,
        scale: 1.0,
        exp: 585,
        job: 540,
        hp: 480,
        sp: 999,
        mp: 999,
        def: [10, 6],
        mdef: [3, 45],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10020300, 0, /*null*/
            10020210, 10020400, 0,
        ],
    },
    Monster {
        name: "Loki",
        id: 10130000,
        monster_type: "ANIMAL",
        level: 24,
        scale: 1.0,
        exp: 411,
        job: 207,
        hp: 380,
        sp: 0,
        mp: 0,
        def: [11, 39],
        mdef: [14, 56],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10020300, 61020707, 10020400, 10020210, 61020707, 0,
        ],
    },
    Monster {
        name: "Infinite Loki",
        id: 10130001,
        monster_type: "ANIMAL",
        level: 16,
        scale: 1.0,
        exp: 27,
        job: 27,
        hp: 164,
        sp: 1000,
        mp: 1000,
        def: [10, 15],
        mdef: [10, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Cave Loki",
        id: 10130100,
        monster_type: "ANIMAL",
        level: 25,
        scale: 1.0,
        exp: 384,
        job: 159,
        hp: 420,
        sp: 107,
        mp: 0,
        def: [10, 31],
        mdef: [16, 41],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10020300, 61020707, 10020210, 10020400, 61020707, 10050943,
        ],
    },
    Monster {
        name: "Kabull",
        id: 10130101,
        monster_type: "ANIMAL",
        level: 36,
        scale: 1.2,
        exp: 1485,
        job: 990,
        hp: 480,
        sp: 999,
        mp: 999,
        def: [12, 80],
        mdef: [44, 49],
        properties: Properties {
            fire: 40,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037300, 10020300, 61020707, 10020210, 10020400, 61020707, 10050966,
        ],
    },
    Monster {
        name: "White Fang",
        id: 10130400,
        monster_type: "ANIMAL",
        level: 21,
        scale: 1.0,
        exp: 306,
        job: 189,
        hp: 200,
        sp: 69,
        mp: 0,
        def: [20, 32],
        mdef: [20, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 40,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10020300, 10035602, 10035602, 61020700, 10050925,
        ],
    },
    Monster {
        name: "Giant White Fang",
        id: 10130401,
        monster_type: "ANIMAL",
        level: 50,
        scale: 2.5,
        exp: 306,
        job: 189,
        hp: 500000,
        sp: 89,
        mp: 203,
        def: [1, 200],
        mdef: [65, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Greyhound",
        id: 10130700,
        monster_type: "ANIMAL",
        level: 30,
        scale: 1.0,
        exp: 990,
        job: 360,
        hp: 460,
        sp: 999,
        mp: 999,
        def: [12, 54],
        mdef: [6, 32],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10037300, 10006400, 10020300, 90000043, 10020210, 61020700, 0,
        ],
    },
    Monster {
        name: "Infinite Greyhound",
        id: 10130701,
        monster_type: "ANIMAL",
        level: 38,
        scale: 1.0,
        exp: 405,
        job: 225,
        hp: 910,
        sp: 1000,
        mp: 1000,
        def: [10, 45],
        mdef: [10, 57],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "White Wolf",
        id: 10136000,
        monster_type: "ANIMAL",
        level: 24,
        scale: 0.7,
        exp: 411,
        job: 207,
        hp: 380,
        sp: 0,
        mp: 0,
        def: [11, 39],
        mdef: [14, 56],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10020300, 90000043, 10020400, 10020210, 10020409, 0,
        ],
    },
    Monster {
        name: "Loner White Wolf",
        id: 10136001,
        monster_type: "ANIMAL_BOSS",
        level: 50,
        scale: 3.5,
        exp: 12600,
        job: 16200,
        hp: 120000,
        sp: 500,
        mp: 500,
        def: [25, 45],
        mdef: [15, 45],
        properties: Properties {
            fire: 0,
            water: 40,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            10006400, 10020300, 10020400, 10020210, 50037300, 10020409, 50037300, 0,
        ],
    },
    Monster {
        name: "White Wolf",
        id: 10136002,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 50,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 7500,
        sp: 0,
        mp: 0,
        def: [15, 110],
        mdef: [30, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "White WolfA",
        id: 10136010,
        monster_type: "ANIMAL",
        level: 24,
        scale: 0.7,
        exp: 411,
        job: 207,
        hp: 380,
        sp: 0,
        mp: 0,
        def: [11, 39],
        mdef: [14, 56],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10020300, 90000043, 10020400, 10020210, 10020409, 0,
        ],
    },
    Monster {
        name: "Gold Wolf",
        id: 10136011,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 50,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 8500,
        sp: 0,
        mp: 0,
        def: [15, 80],
        mdef: [60, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "White WolfB",
        id: 10136009,
        monster_type: "ANIMAL",
        level: 24,
        scale: 0.7,
        exp: 411,
        job: 207,
        hp: 380,
        sp: 0,
        mp: 0,
        def: [11, 39],
        mdef: [14, 56],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10020300, 90000043, 10020400, 10020210, 10020409, 0,
        ],
    },
    Monster {
        name: "Hell King",
        id: 10136900,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 68,
        scale: 3.5,
        exp: 265500,
        job: 162000,
        hp: 660000,
        sp: 500,
        mp: 500,
        def: [55, 350],
        mdef: [65, 350],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            10009500, 10050300, 10009600, 10020755, 10050350, 50037300, 10049500, 10051007,
        ],
    },
    Monster {
        name: "Magic Wolf",
        id: 10136901,
        monster_type: "ANIMAL",
        level: 64,
        scale: 1.3,
        exp: 4050,
        job: 2904,
        hp: 2500,
        sp: 250,
        mp: 250,
        def: [11, 220],
        mdef: [70, 135],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 70,
        },
        drop_ids: [
            0, /*null*/
            10037300, 10006400, 10020300, 0, /*null*/
            10020210, 10020400, 0,
        ],
    },
    Monster {
        name: "Hell King",
        id: 10136902,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 68,
        scale: 3.5,
        exp: 265500,
        job: 162000,
        hp: 660000,
        sp: 500,
        mp: 500,
        def: [55, 350],
        mdef: [65, 350],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10051007,
        ],
    },
    Monster {
        name: "Demon Wolf",
        id: 10136903,
        monster_type: "ANIMAL",
        level: 64,
        scale: 1.3,
        exp: 7200,
        job: 7650,
        hp: 3500,
        sp: 250,
        mp: 250,
        def: [11, 220],
        mdef: [70, 135],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10037300, 10006400, 10020300, 0, /*null*/
            10020210, 10020400, 0,
        ],
    },
    Monster {
        name: "Gigo",
        id: 10140000,
        monster_type: "BIRD",
        level: 41,
        scale: 1.0,
        exp: 1683,
        job: 1512,
        hp: 665,
        sp: 100,
        mp: 100,
        def: [30, 40],
        mdef: [30, 54],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10007700, 10007650, 90000044, 10007650, 90000044, 10050987,
        ],
    },
    Monster {
        name: "Infinite Gigo",
        id: 10140001,
        monster_type: "BIRD",
        level: 42,
        scale: 1.0,
        exp: 333,
        job: 162,
        hp: 720,
        sp: 100,
        mp: 100,
        def: [30, 40],
        mdef: [40, 154],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blue Gigo",
        id: 10140300,
        monster_type: "BIRD",
        level: 37,
        scale: 1.0,
        exp: 1395,
        job: 1080,
        hp: 400,
        sp: 255,
        mp: 255,
        def: [30, 40],
        mdef: [50, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 40,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10019900, 10018310, 10006400, 10018310, 0,
        ],
    },
    Monster {
        name: "Thunderbird",
        id: 10140400,
        monster_type: "BIRD",
        level: 37,
        scale: 1.0,
        exp: 1395,
        job: 1080,
        hp: 400,
        sp: 255,
        mp: 255,
        def: [30, 40],
        mdef: [50, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 40,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10019900, 10018310, 10006400, 10018310, 10050957,
        ],
    },
    Monster {
        name: "Infinite Thunderbird",
        id: 10140401,
        monster_type: "BIRD",
        level: 53,
        scale: 1.0,
        exp: 1710,
        job: 1104,
        hp: 2000,
        sp: 340,
        mp: 340,
        def: [35, 130],
        mdef: [20, 98],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 50,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Vulture",
        id: 10140700,
        monster_type: "BIRD",
        level: 36,
        scale: 1.0,
        exp: 2250,
        job: 2385,
        hp: 2650,
        sp: 255,
        mp: 255,
        def: [30, 40],
        mdef: [50, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 40,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018209, 10007700, 90000046, 0, /*null*/
            10020400, 90000046, 0,
        ],
    },
    Monster {
        name: "Tropical Gigo",
        id: 10140800,
        monster_type: "BIRD",
        level: 78,
        scale: 1.0,
        exp: 11025,
        job: 11025,
        hp: 6500,
        sp: 255,
        mp: 255,
        def: [30, 300],
        mdef: [30, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018204, 10006400, 10052900, 10052900, 10018350, 50040150, 0,
        ],
    },
    Monster {
        name: "Night Stalk",
        id: 10140900,
        monster_type: "BIRD",
        level: 55,
        scale: 1.0,
        exp: 3483,
        job: 2940,
        hp: 2550,
        sp: 255,
        mp: 255,
        def: [15, 115],
        mdef: [45, 165],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007800, 10007700, 10007650, 10049000, 90000044, 10018310, 0,
        ],
    },
    Monster {
        name: "Brown Gigo",
        id: 10141200,
        monster_type: "BIRD",
        level: 37,
        scale: 1.0,
        exp: 1395,
        job: 1080,
        hp: 400,
        sp: 255,
        mp: 255,
        def: [30, 40],
        mdef: [50, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 40,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10019900, 10018310, 10006400, 10018310, 0,
        ],
    },
    Monster {
        name: "White Gigo",
        id: 10141300,
        monster_type: "BIRD",
        level: 41,
        scale: 1.0,
        exp: 1683,
        job: 1512,
        hp: 665,
        sp: 100,
        mp: 100,
        def: [30, 40],
        mdef: [30, 54],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10007700, 10007650, 90000044, 10007650, 90000044, 10050987,
        ],
    },
    Monster {
        name: "Roque Bird",
        id: 10146000,
        monster_type: "BIRD",
        level: 37,
        scale: 0.6,
        exp: 1395,
        job: 1080,
        hp: 400,
        sp: 255,
        mp: 255,
        def: [30, 40],
        mdef: [50, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 40,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10019900, 0, /*null*/
            10006400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Roque BirdA",
        id: 10146010,
        monster_type: "BIRD",
        level: 37,
        scale: 0.6,
        exp: 1395,
        job: 1080,
        hp: 400,
        sp: 255,
        mp: 255,
        def: [30, 40],
        mdef: [50, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 40,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10019900, 0, /*null*/
            10006400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Roque BirdB",
        id: 10146014,
        monster_type: "BIRD",
        level: 37,
        scale: 0.6,
        exp: 1395,
        job: 1080,
        hp: 400,
        sp: 255,
        mp: 255,
        def: [30, 40],
        mdef: [50, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 40,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10019900, 0, /*null*/
            10006400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Jerev",
        id: 10150000,
        monster_type: "ANIMAL",
        level: 23,
        scale: 1.0,
        exp: 288,
        job: 288,
        hp: 190,
        sp: 55,
        mp: 0,
        def: [33, 33],
        mdef: [77, 7],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002202, 10004200, 10019900, 10049000, 10024800, 10009600, 10050927,
        ],
    },
    Monster {
        name: "Infinite Jerev",
        id: 10150001,
        monster_type: "ANIMAL",
        level: 23,
        scale: 1.0,
        exp: 90,
        job: 27,
        hp: 148,
        sp: 1000,
        mp: 1000,
        def: [10, 10],
        mdef: [10, 23],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Ranch Horse",
        id: 10150002,
        monster_type: "ANIMAL_NOTOUCH",
        level: 23,
        scale: 1.0,
        exp: 288,
        job: 288,
        hp: 190,
        sp: 55,
        mp: 0,
        def: [33, 33],
        mdef: [77, 7],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wild Horse",
        id: 10150003,
        monster_type: "ANIMAL_NOTOUCH",
        level: 23,
        scale: 1.0,
        exp: 288,
        job: 288,
        hp: 190,
        sp: 55,
        mp: 0,
        def: [33, 33],
        mdef: [77, 7],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Solar Jerev",
        id: 10155000,
        monster_type: "ANIMAL",
        level: 40,
        scale: 1.0,
        exp: 2880,
        job: 1980,
        hp: 1550,
        sp: 255,
        mp: 255,
        def: [10, 80],
        mdef: [35, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10051500, 10020201, 10000604, 0, /*null*/
            10037300, 10009500, 0,
        ],
    },
    Monster {
        name: "Living Dead",
        id: 10180000,
        monster_type: "UNDEAD",
        level: 29,
        scale: 1.0,
        exp: 564,
        job: 564,
        hp: 520,
        sp: 77,
        mp: 0,
        def: [3, 47],
        mdef: [20, 31],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10000202, 10024900, 10049000, 10009500, 10009600, 10050947,
        ],
    },
    Monster {
        name: "Infinite Living Dead",
        id: 10180002,
        monster_type: "UNDEAD",
        level: 25,
        scale: 1.0,
        exp: 90,
        job: 45,
        hp: 850,
        sp: 1000,
        mp: 1000,
        def: [10, 46],
        mdef: [10, 75],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 5,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Hydra",
        id: 10180003,
        monster_type: "UNDEAD",
        level: 50,
        scale: 1.0,
        exp: 3759,
        job: 3636,
        hp: 1700,
        sp: 0,
        mp: 0,
        def: [15, 234],
        mdef: [31, 166],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 0,
            earth: 60,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10031800, 0, /*null*/
            0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Evil Dead",
        id: 10180100,
        monster_type: "UNDEAD",
        level: 51,
        scale: 1.0,
        exp: 3834,
        job: 3714,
        hp: 1980,
        sp: 255,
        mp: 255,
        def: [20, 95],
        mdef: [40, 170],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10015302, 10034508, 10024900, 0, /*null*/
            10015302, 10009600, 0,
        ],
    },
    Monster {
        name: "Summon Evil",
        id: 10180101,
        monster_type: "UNDEAD",
        level: 70,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 600,
        sp: 255,
        mp: 255,
        def: [10, 140],
        mdef: [30, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Deimos",
        id: 10180102,
        monster_type: "UNDEAD",
        level: 14,
        scale: 1.0,
        exp: 114,
        job: 69,
        hp: 135,
        sp: 0,
        mp: 0,
        def: [14, 24],
        mdef: [14, 25],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031800, 0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Ruuvnan",
        id: 10180500,
        monster_type: "UNDEAD",
        level: 51,
        scale: 1.0,
        exp: 3834,
        job: 3714,
        hp: 1980,
        sp: 255,
        mp: 255,
        def: [20, 95],
        mdef: [40, 170],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10015302, 10034508, 10024900, 0, /*null*/
            10015302, 10009600, 0,
        ],
    },
    Monster {
        name: "Ishtar",
        id: 10180501,
        monster_type: "UNDEAD",
        level: 25,
        scale: 1.0,
        exp: 270,
        job: 180,
        hp: 310,
        sp: 0,
        mp: 0,
        def: [8, 29],
        mdef: [35, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 40,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031800, 0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Undead Panda",
        id: 10181000,
        monster_type: "UNDEAD",
        level: 22,
        scale: 1.0,
        exp: 261,
        job: 216,
        hp: 280,
        sp: 255,
        mp: 255,
        def: [50, 100],
        mdef: [50, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 65,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10034508, 0, /*null*/
            0, /*null*/
            10009500, 0,
        ],
    },
    Monster {
        name: "Nemaka",
        id: 10181001,
        monster_type: "UNDEAD",
        level: 75,
        scale: 1.0,
        exp: 11025,
        job: 11250,
        hp: 4750,
        sp: 0,
        mp: 0,
        def: [20, 220],
        mdef: [50, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10031800, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Skeleton",
        id: 10190000,
        monster_type: "UNDEAD",
        level: 32,
        scale: 1.0,
        exp: 1035,
        job: 765,
        hp: 444,
        sp: 105,
        mp: 0,
        def: [50, 80],
        mdef: [44, 44],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 70,
        },
        drop_ids: [
            0, /*null*/
            10006600, 10024900, 10019900, 10049000, 10045001, 10009600, 10050935,
        ],
    },
    Monster {
        name: "Infinite Skeleton",
        id: 10190001,
        monster_type: "UNDEAD",
        level: 20,
        scale: 1.0,
        exp: 81,
        job: 9,
        hp: 1250,
        sp: 1000,
        mp: 1000,
        def: [10, 52],
        mdef: [10, 76],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 5,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Skeleton Raphael",
        id: 10190100,
        monster_type: "UNDEAD",
        level: 52,
        scale: 1.0,
        exp: 4635,
        job: 3150,
        hp: 2250,
        sp: 255,
        mp: 255,
        def: [20, 100],
        mdef: [40, 75],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10006652, 10024900, 10034502, 0, /*null*/
            10019900, 10009600, 0,
        ],
    },
    Monster {
        name: "Mummy",
        id: 10200000,
        monster_type: "UNDEAD",
        level: 27,
        scale: 1.0,
        exp: 429,
        job: 384,
        hp: 420,
        sp: 89,
        mp: 0,
        def: [55, 10],
        mdef: [77, 44],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10024900, 10021300, 10045901, 10049000, 10021303, 50011400, 10050934,
        ],
    },
    Monster {
        name: "Dried Corpse",
        id: 10200002,
        monster_type: "UNDEAD",
        level: 26,
        scale: 1.2,
        exp: 339,
        job: 339,
        hp: 360,
        sp: 999,
        mp: 999,
        def: [10, 55],
        mdef: [10, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            10024900, 10021300, 10045901, 10049000, 10021303, 50011400, 10050945,
        ],
    },
    Monster {
        name: "Infinite Dried Corpse",
        id: 10200003,
        monster_type: "UNDEAD",
        level: 27,
        scale: 1.0,
        exp: 90,
        job: 72,
        hp: 800,
        sp: 1000,
        mp: 1000,
        def: [10, 46],
        mdef: [10, 71],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Dried Corpse",
        id: 10200004,
        monster_type: "UNDEAD",
        level: 40,
        scale: 1.0,
        exp: 405,
        job: 405,
        hp: 1250,
        sp: 1000,
        mp: 1000,
        def: [10, 56],
        mdef: [10, 56],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Smelman",
        id: 10200005,
        monster_type: "UNDEAD",
        level: 46,
        scale: 1.2,
        exp: 2997,
        job: 2499,
        hp: 2000,
        sp: 89,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10024900, 10021300, 10045901, 0, /*null*/
            10021303, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ancient Death Messenger",
        id: 10200100,
        monster_type: "UNDEAD",
        level: 27,
        scale: 1.0,
        exp: 429,
        job: 384,
        hp: 420,
        sp: 89,
        mp: 0,
        def: [55, 10],
        mdef: [77, 44],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10024900, 10021300, 10045901, 10049000, 10021303, 50011400, 0,
        ],
    },
    Monster {
        name: "Baneful Mummy",
        id: 10200200,
        monster_type: "UNDEAD",
        level: 37,
        scale: 1.0,
        exp: 1620,
        job: 1440,
        hp: 620,
        sp: 89,
        mp: 0,
        def: [25, 60],
        mdef: [25, 15],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            10015302, 10003200, 10034507, 10021303, 50090000, 50090000, 50011400, 0,
        ],
    },
    Monster {
        name: "Summon Mummy",
        id: 10200400,
        monster_type: "UNDEAD",
        level: 70,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 450,
        sp: 89,
        mp: 0,
        def: [10, 130],
        mdef: [30, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Earth Spider",
        id: 10210000,
        monster_type: "INSECT",
        level: 16,
        scale: 1.0,
        exp: 141,
        job: 132,
        hp: 225,
        sp: 77,
        mp: 0,
        def: [23, 31],
        mdef: [40, 29],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10045500, 10019800, 90000044, 10024002, 10019804, 10024001, 10050917,
        ],
    },
    Monster {
        name: "Giant Spider",
        id: 10210001,
        monster_type: "INSECT",
        level: 27,
        scale: 2.0,
        exp: 540,
        job: 450,
        hp: 2400,
        sp: 77,
        mp: 0,
        def: [25, 34],
        mdef: [42, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10045500, 10019800, 10067300, 10049000, 10019804, 10024001, 0,
        ],
    },
    Monster {
        name: "Tarantula",
        id: 10210002,
        monster_type: "INSECT",
        level: 27,
        scale: 1.0,
        exp: 450,
        job: 360,
        hp: 440,
        sp: 999,
        mp: 999,
        def: [10, 42],
        mdef: [35, 90],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10045500, 10019800, 90000044, 10024002, 10019804, 10024001, 10050960,
        ],
    },
    Monster {
        name: "Infinite Tarantula",
        id: 10210003,
        monster_type: "INSECT",
        level: 21,
        scale: 1.0,
        exp: 72,
        job: 27,
        hp: 560,
        sp: 1000,
        mp: 1000,
        def: [10, 38],
        mdef: [10, 65],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Earth Spider",
        id: 10210004,
        monster_type: "INSECT",
        level: 49,
        scale: 1.0,
        exp: 990,
        job: 360,
        hp: 1500,
        sp: 77,
        mp: 0,
        def: [10, 150],
        mdef: [20, 84],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Earth Spider",
        id: 10210005,
        monster_type: "INSECT",
        level: 16,
        scale: 1.0,
        exp: 141,
        job: 132,
        hp: 225,
        sp: 77,
        mp: 0,
        def: [23, 31],
        mdef: [40, 29],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10045500, 10019800, 90000044, 10024002, 10019804, 10024001, 10050917,
        ],
    },
    Monster {
        name: "Giant Spider",
        id: 10210006,
        monster_type: "INSECT",
        level: 27,
        scale: 2.0,
        exp: 540,
        job: 450,
        hp: 2400,
        sp: 77,
        mp: 0,
        def: [25, 34],
        mdef: [42, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10045500, 10019800, 10024002, 10049000, 10019804, 10024001, 0,
        ],
    },
    Monster {
        name: "Koganegumo",
        id: 10210100,
        monster_type: "INSECT",
        level: 77,
        scale: 1.0,
        exp: 11250,
        job: 10800,
        hp: 5000,
        sp: 77,
        mp: 0,
        def: [30, 151],
        mdef: [80, 200],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10037501, 10015708, 10024002, 10037505, 90000046, 0,
        ],
    },
    Monster {
        name: "Phobos",
        id: 10210101,
        monster_type: "INSECT",
        level: 16,
        scale: 1.0,
        exp: 141,
        job: 132,
        hp: 225,
        sp: 0,
        mp: 0,
        def: [23, 31],
        mdef: [40, 29],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10024009, 0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "G. Skulatula",
        id: 10210200,
        monster_type: "INSECT",
        level: 91,
        scale: 1.0,
        exp: 18900,
        job: 18450,
        hp: 12000,
        sp: 77,
        mp: 0,
        def: [35, 300],
        mdef: [98, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 10,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10045550, 10015300, 10024002, 10014250, 90000046, 0,
        ],
    },
    Monster {
        name: "G. Skulatula",
        id: 10210201,
        monster_type: "INSECT",
        level: 24,
        scale: 1.0,
        exp: 207,
        job: 384,
        hp: 300,
        sp: 0,
        mp: 0,
        def: [24, 10],
        mdef: [34, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10024009, 0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Black Ghost Spider",
        id: 10210900,
        monster_type: "INSECT_BOSS_SKILL",
        level: 49,
        scale: 5.5,
        exp: 15300,
        job: 24300,
        hp: 14500,
        sp: 200,
        mp: 250,
        def: [10, 200],
        mdef: [20, 85],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10045500, 10045500, 10038102, 10038102, 10019801, 10034502, 50011109, 0,
        ],
    },
    Monster {
        name: "Black Ghost Spider",
        id: 10210901,
        monster_type: "INSECT_BOSS_SKILL",
        level: 49,
        scale: 5.5,
        exp: 15300,
        job: 24300,
        hp: 14500,
        sp: 77,
        mp: 0,
        def: [10, 200],
        mdef: [20, 84],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10045500, 10045500, 10038102, 10038102, 10019801, 10034502, 50011109, 0,
        ],
    },
    Monster {
        name: "Black Ghost Spider",
        id: 10210902,
        monster_type: "INSECT_BOSS_SKILL",
        level: 49,
        scale: 5.5,
        exp: 6300,
        job: 10800,
        hp: 14500,
        sp: 77,
        mp: 0,
        def: [10, 100],
        mdef: [20, 84],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10045500, 10045500, 10038102, 10038102, 0, /*null*/
            10034502, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nix",
        id: 10210903,
        monster_type: "INSECT",
        level: 44,
        scale: 1.0,
        exp: 1620,
        job: 2175,
        hp: 700,
        sp: 0,
        mp: 0,
        def: [56, 45],
        mdef: [10, 40],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 45,
        },
        drop_ids: [
            0, /*null*/
            10024009, 0, /*null*/
            0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Midnight Spider",
        id: 10211000,
        monster_type: "INSECT_UNITE",
        level: 62,
        scale: 1.5,
        exp: 3375,
        job: 4500,
        hp: 2550,
        sp: 150,
        mp: 170,
        def: [10, 150],
        mdef: [20, 84],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10045500, 10019800, 90000044, 10024002, 10019804, 10024001, 0,
        ],
    },
    Monster {
        name: "Hi'iaka Spider",
        id: 10211001,
        monster_type: "INSECT",
        level: 75,
        scale: 1.0,
        exp: 10575,
        job: 11250,
        hp: 4600,
        sp: 0,
        mp: 0,
        def: [100, 155],
        mdef: [25, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [
            10024009, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            50030850, 50030850, 0,
        ],
    },
    Monster {
        name: "Yaksa Spider",
        id: 10211400,
        monster_type: "INSECT_BOSS_SKILL",
        level: 50,
        scale: 5.5,
        exp: 24300,
        job: 19800,
        hp: 16550,
        sp: 250,
        mp: 300,
        def: [10, 200],
        mdef: [20, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10003200, 10003200, 10019807, 10019807, 10024002, 10019800, 50011111, 0,
        ],
    },
    Monster {
        name: "Muskes Binnet",
        id: 10211500,
        monster_type: "INSECT",
        level: 13,
        scale: 1.0,
        exp: 240,
        job: 384,
        hp: 400,
        sp: 77,
        mp: 0,
        def: [15, 20],
        mdef: [40, 29],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003200, 10019900, 0, /*null*/
            90000046, 10019800, 90000046, 0,
        ],
    },
    Monster {
        name: "Muskes Binnet",
        id: 10211501,
        monster_type: "INSECT",
        level: 1,
        scale: 1.0,
        exp: 240,
        job: 384,
        hp: 400,
        sp: 77,
        mp: 0,
        def: [15, 20],
        mdef: [40, 29],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wisp",
        id: 10220000,
        monster_type: "MAGIC_CREATURE",
        level: 24,
        scale: 1.0,
        exp: 207,
        job: 384,
        hp: 300,
        sp: 58,
        mp: 97,
        def: [24, 10],
        mdef: [34, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10001150, 10001400, 10049000, 10033300, 10009600, 10050942,
        ],
    },
    Monster {
        name: "Infinite Wisp",
        id: 10220001,
        monster_type: "MAGIC_CREATURE",
        level: 21,
        scale: 1.0,
        exp: 63,
        job: 36,
        hp: 315,
        sp: 1000,
        mp: 1000,
        def: [10, 42],
        mdef: [10, 36],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Andante",
        id: 10220100,
        monster_type: "MAGIC_CREATURE",
        level: 37,
        scale: 1.0,
        exp: 360,
        job: 180,
        hp: 650,
        sp: 1000,
        mp: 1000,
        def: [10, 54],
        mdef: [10, 68],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Andante",
        id: 10220101,
        monster_type: "MAGIC_CREATURE",
        level: 41,
        scale: 1.0,
        exp: 1206,
        job: 1629,
        hp: 1800,
        sp: 100,
        mp: 100,
        def: [50, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10001400, 10001111, 0, /*null*/
            10001111, 10009600, 10050988,
        ],
    },
    Monster {
        name: "Geister",
        id: 10220200,
        monster_type: "MAGIC_CREATURE",
        level: 57,
        scale: 1.0,
        exp: 4725,
        job: 3285,
        hp: 2100,
        sp: 58,
        mp: 97,
        def: [25, 100],
        mdef: [24, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10001150, 10001400, 0, /*null*/
            10033300, 90000044, 0,
        ],
    },
    Monster {
        name: "Photon",
        id: 10220500,
        monster_type: "MAGIC_CREATURE",
        level: 70,
        scale: 1.0,
        exp: 8100,
        job: 9450,
        hp: 3400,
        sp: 300,
        mp: 300,
        def: [10, 155],
        mdef: [60, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10035000, 90000045, 0, /*null*/
            10035000, 90000045, 0,
        ],
    },
    Monster {
        name: "Bubi",
        id: 10220700,
        monster_type: "MAGIC_CREATURE",
        level: 28,
        scale: 1.0,
        exp: 360,
        job: 540,
        hp: 320,
        sp: 84,
        mp: 185,
        def: [15, 45],
        mdef: [38, 55],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10035000, 10001400, 10049000, 10009500, 10035600, 10050948,
        ],
    },
    Monster {
        name: "Infinite Bubi",
        id: 10220701,
        monster_type: "MAGIC_CREATURE",
        level: 16,
        scale: 1.0,
        exp: 36,
        job: 18,
        hp: 92,
        sp: 1000,
        mp: 1000,
        def: [10, 29],
        mdef: [15, 50],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Giant Bubi",
        id: 10220702,
        monster_type: "MAGIC_CREATURE",
        level: 50,
        scale: 3.0,
        exp: 360,
        job: 540,
        hp: 500000,
        sp: 89,
        mp: 203,
        def: [1, 200],
        mdef: [65, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10035000, 10001400, 10049000, 10009500, 10035600, 0,
        ],
    },
    Monster {
        name: "Ectoplasma",
        id: 10221000,
        monster_type: "MAGIC_CREATURE",
        level: 74,
        scale: 1.0,
        exp: 10350,
        job: 7560,
        hp: 3500,
        sp: 58,
        mp: 97,
        def: [20, 10],
        mdef: [30, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10001150, 10001400, 0, /*null*/
            10033300, 90000046, 0,
        ],
    },
    Monster {
        name: "Pink Wisp",
        id: 10221500,
        monster_type: "MAGIC_CREATURE",
        level: 24,
        scale: 1.0,
        exp: 207,
        job: 384,
        hp: 300,
        sp: 58,
        mp: 97,
        def: [24, 10],
        mdef: [34, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10001150, 10001400, 10049000, 10033300, 10009600, 0,
        ],
    },
    Monster {
        name: "Clone Nekomata",
        id: 10222000,
        monster_type: "MAGIC_CREATURE",
        level: 19,
        scale: 1.0,
        exp: 225,
        job: 225,
        hp: 280,
        sp: 1000,
        mp: 1000,
        def: [10, 34],
        mdef: [15, 55],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mini Demon",
        id: 10240000,
        monster_type: "MAGIC_CREATURE",
        level: 31,
        scale: 1.2,
        exp: 540,
        job: 810,
        hp: 280,
        sp: 255,
        mp: 255,
        def: [8, 28],
        mdef: [50, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10003200, 10009100, 90000046, 10013350, 10000106, 10009500, 10050954,
        ],
    },
    Monster {
        name: "Infinite Mini Demon",
        id: 10240001,
        monster_type: "MAGIC_CREATURE",
        level: 41,
        scale: 1.2,
        exp: 90,
        job: 360,
        hp: 800,
        sp: 255,
        mp: 255,
        def: [20, 48],
        mdef: [38, 60],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Black Mini Demon",
        id: 10241000,
        monster_type: "MAGIC_CREATURE",
        level: 40,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 800,
        sp: 100,
        mp: 100,
        def: [15, 110],
        mdef: [45, 228],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Black Mini Demon",
        id: 10241001,
        monster_type: "MAGIC_CREATURE",
        level: 54,
        scale: 1.2,
        exp: 810,
        job: 900,
        hp: 1780,
        sp: 100,
        mp: 100,
        def: [30, 110],
        mdef: [30, 235],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Purple Mini Demon",
        id: 10240200,
        monster_type: "MAGIC_CREATURE",
        level: 31,
        scale: 1.2,
        exp: 540,
        job: 810,
        hp: 280,
        sp: 255,
        mp: 255,
        def: [8, 28],
        mdef: [50, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10003200, 10009100, 90000046, 10013350, 10000106, 10009500, 10050954,
        ],
    },
    Monster {
        name: "Fiendish Imp",
        id: 10240300,
        monster_type: "MAGIC_CREATURE",
        level: 24,
        scale: 1.2,
        exp: 675,
        job: 1518,
        hp: 630,
        sp: 255,
        mp: 255,
        def: [10, 30],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10019900, 0, /*null*/
            90000045, 10001200, 90000045, 0,
        ],
    },
    Monster {
        name: "Lazy Imp",
        id: 10240301,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.2,
        exp: 675,
        job: 1518,
        hp: 630,
        sp: 255,
        mp: 255,
        def: [10, 30],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "White Petite Demon",
        id: 10240400,
        monster_type: "MAGIC_CREATURE",
        level: 53,
        scale: 1.2,
        exp: 3105,
        job: 3897,
        hp: 1850,
        sp: 100,
        mp: 100,
        def: [16, 80],
        mdef: [50, 260],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009101, 10001200, 10009107, 10049000, 10000106, 10009500, 10050990,
        ],
    },
    Monster {
        name: "Brat",
        id: 10240600,
        monster_type: "MAGIC_CREATURE",
        level: 15,
        scale: 1.2,
        exp: 522,
        job: 1017,
        hp: 355,
        sp: 255,
        mp: 255,
        def: [10, 20],
        mdef: [10, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10019900, 0, /*null*/
            90000045, 10001200, 90000045, 0,
        ],
    },
    Monster {
        name: "Lacking Devil",
        id: 10240601,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.2,
        exp: 522,
        job: 1017,
        hp: 355,
        sp: 255,
        mp: 255,
        def: [10, 20],
        mdef: [10, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "White Familiar",
        id: 10241400,
        monster_type: "MAGIC_CREATURE",
        level: 31,
        scale: 1.2,
        exp: 540,
        job: 810,
        hp: 280,
        sp: 255,
        mp: 255,
        def: [8, 28],
        mdef: [50, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10003200, 10009100, 90000046, 10013350, 10000106, 10009500, 10050954,
        ],
    },
    Monster {
        name: "Death",
        id: 10250000,
        monster_type: "UNDEAD",
        level: 41,
        scale: 1.0,
        exp: 2250,
        job: 1440,
        hp: 444,
        sp: 130,
        mp: 367,
        def: [20, 90],
        mdef: [70, 95],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006600, 0, /*null*/
            10067300, 10049000, 10042900, 10017901, 10050958,
        ],
    },
    Monster {
        name: "Infinite Death",
        id: 10250001,
        monster_type: "UNDEAD",
        level: 35,
        scale: 1.0,
        exp: 150,
        job: 132,
        hp: 1150,
        sp: 1000,
        mp: 1000,
        def: [10, 63],
        mdef: [10, 77],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Ghuz",
        id: 10250002,
        monster_type: "UNDEAD_BOSS",
        level: 45,
        scale: 1.8,
        exp: 7650,
        job: 7650,
        hp: 3500,
        sp: 1000,
        mp: 1000,
        def: [10, 80],
        mdef: [25, 101],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Ghuz",
        id: 10250003,
        monster_type: "UNDEAD",
        level: 43,
        scale: 1.2,
        exp: 16200,
        job: 8100,
        hp: 3200,
        sp: 130,
        mp: 367,
        def: [30, 100],
        mdef: [5, 101],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10006600, 0, /*null*/
            10019900, 0, /*null*/
            10042900, 10017901, 0,
        ],
    },
    Monster {
        name: "Death",
        id: 10250004,
        monster_type: "UNDEAD_NOTOUCH",
        level: 95,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 444,
        sp: 130,
        mp: 367,
        def: [20, 90],
        mdef: [70, 95],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ghuz",
        id: 10250005,
        monster_type: "UNDEAD_NOTOUCH",
        level: 95,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 3200,
        sp: 130,
        mp: 367,
        def: [30, 100],
        mdef: [5, 101],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fate",
        id: 10250006,
        monster_type: "UNDEAD_BOSS_SKILL_NOTPTDROPRANGE",
        level: 50,
        scale: 3.0,
        exp: 36000,
        job: 31500,
        hp: 16000,
        sp: 130,
        mp: 367,
        def: [30, 100],
        mdef: [5, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            10011601, 10011601, 10011601, 10011601, 10011601, 10011601, 10011601, 0,
        ],
    },
    Monster {
        name: "Saluter",
        id: 10250007,
        monster_type: "UNDEAD",
        level: 49,
        scale: 1.4,
        exp: 3384,
        job: 2934,
        hp: 2800,
        sp: 130,
        mp: 367,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10006600, 0, /*null*/
            10067300, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Bloody",
        id: 10250100,
        monster_type: "UNDEAD",
        level: 0,
        scale: 1.0,
        exp: 105,
        job: 105,
        hp: 500,
        sp: 140,
        mp: 400,
        def: [99, 2],
        mdef: [50, 11],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10019900, 10049000, 10009500, 10009600, 0,
        ],
    },
    Monster {
        name: "Grim Reaper",
        id: 10250400,
        monster_type: "UNDEAD_BOSS_SKILL",
        level: 60,
        scale: 3.0,
        exp: 49500,
        job: 58500,
        hp: 24000,
        sp: 130,
        mp: 367,
        def: [30, 220],
        mdef: [35, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            10001200, 10009101, 10009101, 10029300, 50040500, 10000604, 60051800, 0,
        ],
    },
    Monster {
        name: "Grim Reaper",
        id: 10250401,
        monster_type: "UNDEAD_BOSS_SKILL",
        level: 60,
        scale: 3.0,
        exp: 49500,
        job: 58500,
        hp: 24000,
        sp: 130,
        mp: 367,
        def: [30, 220],
        mdef: [35, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            10001200, 10009101, 10009101, 10029300, 50040500, 10000604, 60051800, 0,
        ],
    },
    Monster {
        name: "Grim Reaper",
        id: 10250402,
        monster_type: "UNDEAD_NOTOUCH",
        level: 60,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 24000,
        sp: 130,
        mp: 367,
        def: [30, 220],
        mdef: [35, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Orcus",
        id: 10250500,
        monster_type: "UNDEAD",
        level: 96,
        scale: 1.0,
        exp: 27000,
        job: 29700,
        hp: 21000,
        sp: 260,
        mp: 367,
        def: [35, 200],
        mdef: [28, 128],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10009900, 10009101, 10019900, 10054900, 10009900, 10054900, 0,
        ],
    },
    Monster {
        name: "Nightmare",
        id: 10251000,
        monster_type: "UNDEAD_BOSS_SKILL",
        level: 62,
        scale: 3.0,
        exp: 58500,
        job: 49500,
        hp: 25000,
        sp: 130,
        mp: 367,
        def: [30, 220],
        mdef: [35, 160],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            10001200, 10009101, 10009101, 10029300, 50040500, 10000604, 10054900, 0,
        ],
    },
    Monster {
        name: "Nightmare",
        id: 10251001,
        monster_type: "UNDEAD_NOTOUCH",
        level: 62,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 25000,
        sp: 130,
        mp: 367,
        def: [30, 220],
        mdef: [35, 160],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nightmare",
        id: 10251002,
        monster_type: "UNDEAD_BOSS_SKILL",
        level: 62,
        scale: 3.0,
        exp: 58500,
        job: 49500,
        hp: 25000,
        sp: 130,
        mp: 367,
        def: [30, 220],
        mdef: [35, 160],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Soul Eater",
        id: 10251200,
        monster_type: "UNDEAD_NOTOUCH",
        level: 62,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 10000,
        sp: 130,
        mp: 367,
        def: [30, 220],
        mdef: [35, 160],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nekomata ",
        id: 10260000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Nekomata (Midori)",
        id: 10260050,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Nekomata (Ai)",
        id: 10260051,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Nekomata (Yamabuki)",
        id: 10260052,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Angel Feather",
        id: 10270000,
        monster_type: "MAGIC_CREATURE",
        level: 28,
        scale: 1.0,
        exp: 294,
        job: 519,
        hp: 232,
        sp: 255,
        mp: 255,
        def: [15, 10],
        mdef: [26, 57],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018208, 10001200, 90000045, 10011600, 0, /*null*/
            10000604, 0,
        ],
    },
    Monster {
        name: "Infinite Angel Feather",
        id: 10270001,
        monster_type: "MAGIC_CREATURE",
        level: 47,
        scale: 1.0,
        exp: 294,
        job: 519,
        hp: 900,
        sp: 255,
        mp: 255,
        def: [25, 85],
        mdef: [20, 89],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 25,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Emil Feather R",
        id: 10270100,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL_NOTPTDROPRANGE",
        level: 145,
        scale: 1.0,
        exp: 294,
        job: 519,
        hp: 900,
        sp: 255,
        mp: 255,
        def: [25, 85],
        mdef: [20, 89],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 25,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Emil Feather B",
        id: 10270400,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL_NOTPTDROPRANGE",
        level: 145,
        scale: 1.0,
        exp: 294,
        job: 519,
        hp: 900,
        sp: 255,
        mp: 255,
        def: [25, 85],
        mdef: [20, 89],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 25,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Emil Feather Y",
        id: 10270500,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL_NOTPTDROPRANGE",
        level: 145,
        scale: 1.0,
        exp: 294,
        job: 519,
        hp: 900,
        sp: 255,
        mp: 255,
        def: [25, 85],
        mdef: [20, 89],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 25,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Emil Feather G",
        id: 10270800,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL_NOTPTDROPRANGE",
        level: 145,
        scale: 1.0,
        exp: 294,
        job: 519,
        hp: 900,
        sp: 255,
        mp: 255,
        def: [25, 85],
        mdef: [20, 89],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 25,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dark Feather",
        id: 10271000,
        monster_type: "MAGIC_CREATURE",
        level: 35,
        scale: 1.0,
        exp: 585,
        job: 1530,
        hp: 300,
        sp: 255,
        mp: 255,
        def: [8, 11],
        mdef: [50, 84],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10018209, 10024900, 90000045, 0, /*null*/
            10000604, 10009500, 10050956,
        ],
    },
    Monster {
        name: "Infinite Dark Feather",
        id: 10271001,
        monster_type: "MAGIC_CREATURE",
        level: 43,
        scale: 1.0,
        exp: 135,
        job: 180,
        hp: 800,
        sp: 255,
        mp: 255,
        def: [38, 51],
        mdef: [38, 104],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ominous Feather",
        id: 10271002,
        monster_type: "MAGIC_CREATURE_SKILL",
        level: 1,
        scale: 1.2,
        exp: 405,
        job: 1800,
        hp: 300,
        sp: 255,
        mp: 255,
        def: [8, 11],
        mdef: [50, 84],
        properties: Properties {
            fire: 50,
            water: 50,
            wind: 50,
            earth: 50,
            light: 0,
            dark: 255,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Luminous Feather",
        id: 10275000,
        monster_type: "MAGIC_CREATURE",
        level: 54,
        scale: 1.0,
        exp: 2475,
        job: 2475,
        hp: 2100,
        sp: 255,
        mp: 255,
        def: [10, 95],
        mdef: [40, 165],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018207, 10001200, 10000604, 0, /*null*/
            10018207, 10009600, 0,
        ],
    },
    Monster {
        name: "Chaos Feather",
        id: 10276000,
        monster_type: "MAGIC_CREATURE",
        level: 54,
        scale: 1.0,
        exp: 2475,
        job: 2475,
        hp: 2000,
        sp: 255,
        mp: 255,
        def: [10, 85],
        mdef: [40, 170],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10018207, 10001200, 10000604, 10067300, 10018207, 10009600, 0,
        ],
    },
    Monster {
        name: "Larva",
        id: 10300000,
        monster_type: "INSECT",
        level: 16,
        scale: 0.8,
        exp: 135,
        job: 135,
        hp: 160,
        sp: 55,
        mp: 117,
        def: [21, 17],
        mdef: [15, 25],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10034507, 10001100, 90000046, 10007700, 10002200, 10008450, 0,
        ],
    },
    Monster {
        name: "Infinite Larva",
        id: 10300001,
        monster_type: "INSECT",
        level: 50,
        scale: 0.8,
        exp: 45,
        job: 45,
        hp: 550,
        sp: 55,
        mp: 117,
        def: [10, 70],
        mdef: [20, 86],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Crawler Carrier (White)",
        id: 10300002,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mini Crawler Carrier (White)",
        id: 10300003,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Arcana Jack",
        id: 10310000,
        monster_type: "MAGIC_CREATURE",
        level: 20,
        scale: 1.3,
        exp: 45,
        job: 36,
        hp: 315,
        sp: 1000,
        mp: 1000,
        def: [20, 17],
        mdef: [20, 26],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010005, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Arcana Jack",
        id: 10310001,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 40,
        scale: 1.4,
        exp: 1575,
        job: 1575,
        hp: 800,
        sp: 110,
        mp: 255,
        def: [10, 30],
        mdef: [55, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000108, 10020000, 10019900, 10049000, 10000604, 10019900, 10050976,
        ],
    },
    Monster {
        name: "Infinite Arcana Jack",
        id: 10310002,
        monster_type: "MAGIC_CREATURE",
        level: 20,
        scale: 1.3,
        exp: 45,
        job: 36,
        hp: 315,
        sp: 1000,
        mp: 1000,
        def: [20, 17],
        mdef: [20, 26],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010006, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Jack",
        id: 10310003,
        monster_type: "MAGIC_CREATURE",
        level: 20,
        scale: 1.3,
        exp: 45,
        job: 36,
        hp: 315,
        sp: 1000,
        mp: 1000,
        def: [20, 17],
        mdef: [20, 26],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010007, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Jack",
        id: 10310004,
        monster_type: "MAGIC_CREATURE",
        level: 20,
        scale: 1.3,
        exp: 45,
        job: 36,
        hp: 315,
        sp: 1000,
        mp: 1000,
        def: [20, 17],
        mdef: [20, 26],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010008, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Jack",
        id: 10310005,
        monster_type: "MAGIC_CREATURE",
        level: 20,
        scale: 1.3,
        exp: 45,
        job: 36,
        hp: 315,
        sp: 1000,
        mp: 1000,
        def: [20, 17],
        mdef: [20, 26],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010009, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Arcana Jack",
        id: 10310006,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 38,
        scale: 1.4,
        exp: 0,
        job: 0,
        hp: 800,
        sp: 110,
        mp: 255,
        def: [10, 30],
        mdef: [55, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Queen Card",
        id: 10310007,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 80,
        scale: 2.0,
        exp: 81000,
        job: 81000,
        hp: 150000,
        sp: 110,
        mp: 255,
        def: [25, 200],
        mdef: [20, 200],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000108, 10020000, 10019900, 10049000, 10000604, 10019900, 0,
        ],
    },
    Monster {
        name: "Infinite Arcana Queen",
        id: 10320000,
        monster_type: "MAGIC_CREATURE",
        level: 39,
        scale: 1.4,
        exp: 360,
        job: 360,
        hp: 1000,
        sp: 1000,
        mp: 1000,
        def: [20, 37],
        mdef: [20, 31],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010010, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Arcana Queen",
        id: 10320001,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL",
        level: 41,
        scale: 1.5,
        exp: 2250,
        job: 1125,
        hp: 2000,
        sp: 255,
        mp: 255,
        def: [5, 80],
        mdef: [85, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 1,
        },
        drop_ids: [
            10001200, 10000604, 10067302, 0, /*null*/
            10049000, 10009600, 10017900, 10050978,
        ],
    },
    Monster {
        name: "Infinite Arcana Queen",
        id: 10320002,
        monster_type: "MAGIC_CREATURE",
        level: 39,
        scale: 1.4,
        exp: 360,
        job: 360,
        hp: 1000,
        sp: 1000,
        mp: 1000,
        def: [20, 37],
        mdef: [20, 31],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010011, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Queen",
        id: 10320003,
        monster_type: "MAGIC_CREATURE",
        level: 39,
        scale: 1.4,
        exp: 360,
        job: 360,
        hp: 1000,
        sp: 1000,
        mp: 1000,
        def: [20, 37],
        mdef: [20, 31],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010012, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Queen",
        id: 10320004,
        monster_type: "MAGIC_CREATURE",
        level: 39,
        scale: 1.4,
        exp: 360,
        job: 360,
        hp: 1000,
        sp: 1000,
        mp: 1000,
        def: [20, 37],
        mdef: [20, 31],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010013, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana Queen",
        id: 10320005,
        monster_type: "MAGIC_CREATURE",
        level: 39,
        scale: 1.4,
        exp: 360,
        job: 360,
        hp: 1000,
        sp: 1000,
        mp: 1000,
        def: [20, 37],
        mdef: [20, 31],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [10010014, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Arcana Queen",
        id: 10320006,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 39,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 800,
        sp: 255,
        mp: 255,
        def: [5, 150],
        mdef: [85, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Maze Queen",
        id: 10320007,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 80,
        scale: 2.5,
        exp: 135000,
        job: 135000,
        hp: 300000,
        sp: 255,
        mp: 255,
        def: [25, 200],
        mdef: [20, 200],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            10001200, 10000604, 10019900, 0, /*null*/
            10049000, 10009600, 10017900, 0,
        ],
    },
    Monster {
        name: "Infinite Arcana King",
        id: 10330000,
        monster_type: "MAGIC_CREATURE",
        level: 42,
        scale: 1.0,
        exp: 495,
        job: 495,
        hp: 1250,
        sp: 1000,
        mp: 1000,
        def: [20, 35],
        mdef: [20, 38],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [10010015, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Arcana King",
        id: 10330001,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL",
        level: 46,
        scale: 1.6,
        exp: 179100,
        job: 60975,
        hp: 54000,
        sp: 255,
        mp: 255,
        def: [35, 155],
        mdef: [45, 180],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 40,
            dark: 11,
        },
        drop_ids: [
            10000604, 10019900, 10009500, 10050300, 10049500, 10050300, 10049500, 0,
        ],
    },
    Monster {
        name: "Infinite Arcana King",
        id: 10330002,
        monster_type: "MAGIC_CREATURE",
        level: 42,
        scale: 1.0,
        exp: 495,
        job: 495,
        hp: 1250,
        sp: 1000,
        mp: 1000,
        def: [20, 35],
        mdef: [20, 38],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [10010016, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana King",
        id: 10330003,
        monster_type: "MAGIC_CREATURE",
        level: 42,
        scale: 1.0,
        exp: 495,
        job: 495,
        hp: 1250,
        sp: 1000,
        mp: 1000,
        def: [20, 35],
        mdef: [20, 38],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [10010017, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana King",
        id: 10330004,
        monster_type: "MAGIC_CREATURE",
        level: 42,
        scale: 1.0,
        exp: 495,
        job: 495,
        hp: 1250,
        sp: 1000,
        mp: 1000,
        def: [20, 35],
        mdef: [20, 38],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [10010018, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Arcana King",
        id: 10330005,
        monster_type: "MAGIC_CREATURE",
        level: 42,
        scale: 1.0,
        exp: 495,
        job: 495,
        hp: 1250,
        sp: 1000,
        mp: 1000,
        def: [20, 35],
        mdef: [20, 38],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [10010019, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Arcana King",
        id: 10330006,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 42,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1250,
        sp: 1000,
        mp: 1000,
        def: [20, 35],
        mdef: [20, 38],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Cyclops",
        id: 10340000,
        monster_type: "MAGIC_CREATURE",
        level: 32,
        scale: 0.7,
        exp: 1404,
        job: 1080,
        hp: 840,
        sp: 110,
        mp: 255,
        def: [5, 140],
        mdef: [27, 15],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10050851, 90000044, 10014300, 10019900, 10009500, 10050955,
        ],
    },
    Monster {
        name: "Infinite Cyclops",
        id: 10340001,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 55,
        scale: 1.2,
        exp: 2025,
        job: 450,
        hp: 2500,
        sp: 110,
        mp: 255,
        def: [50, 120],
        mdef: [40, 190],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Post Demon",
        id: 10340200,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 60,
        scale: 1.3,
        exp: 99000,
        job: 76500,
        hp: 24000,
        sp: 110,
        mp: 255,
        def: [40, 140],
        mdef: [50, 200],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 40,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10050851, 90000044, 10014300, 10019900, 10009500, 0,
        ],
    },
    Monster {
        name: "Ogre",
        id: 10340300,
        monster_type: "MAGIC_CREATURE",
        level: 39,
        scale: 0.7,
        exp: 2070,
        job: 1305,
        hp: 2700,
        sp: 140,
        mp: 255,
        def: [5, 10],
        mdef: [0, 5],
        properties: Properties {
            fire: 0,
            water: 80,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10033904, 10000103, 10019900, 10050851, 10019900, 10009500, 10050977,
        ],
    },
    Monster {
        name: "Blocks",
        id: 10340301,
        monster_type: "MAGIC_CREATURE",
        level: 51,
        scale: 0.7,
        exp: 4329,
        job: 3573,
        hp: 4950,
        sp: 100,
        mp: 100,
        def: [13, 27],
        mdef: [15, 53],
        properties: Properties {
            fire: 40,
            water: 0,
            wind: 0,
            earth: 80,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10000108, 10019900, 0, /*null*/
            10019900, 10009500, 10050996,
        ],
    },
    Monster {
        name: "Oceanus",
        id: 10340800,
        monster_type: "MAGIC_CREATURE",
        level: 32,
        scale: 0.7,
        exp: 1080,
        job: 720,
        hp: 600,
        sp: 110,
        mp: 255,
        def: [5, 140],
        mdef: [18, 15],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 40,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10050851, 90000044, 10014300, 10019900, 10009500, 0,
        ],
    },
    Monster {
        name: "Former Demon",
        id: 10341000,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 60,
        scale: 1.3,
        exp: 112500,
        job: 81000,
        hp: 25000,
        sp: 110,
        mp: 255,
        def: [40, 230],
        mdef: [35, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 40,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10050851, 90000044, 10014300, 10019900, 10009500, 0,
        ],
    },
    Monster {
        name: "Great Demon",
        id: 10341500,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 80000,
        sp: 0,
        mp: 0,
        def: [52, 110],
        mdef: [63, 215],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Skeleton Archer",
        id: 10350000,
        monster_type: "UNDEAD",
        level: 35,
        scale: 1.0,
        exp: 1104,
        job: 1149,
        hp: 400,
        sp: 120,
        mp: 255,
        def: [18, 60],
        mdef: [50, 75],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10006600, 10024900, 10011600, 10049000, 10019900, 10009500, 10050972,
        ],
    },
    Monster {
        name: "Infinite Skeleton Archer",
        id: 10350001,
        monster_type: "UNDEAD_BOSS",
        level: 50,
        scale: 1.5,
        exp: 810,
        job: 990,
        hp: 1980,
        sp: 120,
        mp: 255,
        def: [20, 160],
        mdef: [30, 160],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 25,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Skeleton Shooter",
        id: 10350100,
        monster_type: "UNDEAD",
        level: 53,
        scale: 1.0,
        exp: 3879,
        job: 3645,
        hp: 2200,
        sp: 250,
        mp: 255,
        def: [10, 100],
        mdef: [40, 65],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10006652, 10026502, 10026600, 0, /*null*/
            10019900, 10009600, 0,
        ],
    },
    Monster {
        name: "Summon Skeleton",
        id: 10350101,
        monster_type: "UNDEAD",
        level: 70,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 700,
        sp: 250,
        mp: 255,
        def: [10, 160],
        mdef: [60, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Key Bag",
        id: 10360000,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 10360001,
        monster_type: "MAGIC_CREATURE_LVDIFF",
        level: 1,
        scale: 1.0,
        exp: 225,
        job: 225,
        hp: 3000,
        sp: 999,
        mp: 999,
        def: [30, 150],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 10049000, 10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 10360002,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 0,
        mp: 0,
        def: [99, 0],
        mdef: [10, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Takara",
        id: 10370000,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Treasure Box",
        id: 10370001,
        monster_type: "MAGIC_CREATURE_LVDIFF",
        level: 1,
        scale: 1.0,
        exp: 225,
        job: 225,
        hp: 3000,
        sp: 999,
        mp: 999,
        def: [30, 150],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 10049000, 10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Briking Cannon",
        id: 10380000,
        monster_type: "MACHINE",
        level: 49,
        scale: 1.0,
        exp: 2472,
        job: 2583,
        hp: 725,
        sp: 100,
        mp: 100,
        def: [45, 146],
        mdef: [12, 33],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015500, 0, /*null*/
            10023501, 10049000, 0, /*null*/
            10030001, 0,
        ],
    },
    Monster {
        name: "Briking Cannon??",
        id: 10380001,
        monster_type: "MACHINE",
        level: 55,
        scale: 1.0,
        exp: 9450,
        job: 5184,
        hp: 3750,
        sp: 100,
        mp: 100,
        def: [30, 180],
        mdef: [45, 249],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking Cannon",
        id: 10380002,
        monster_type: "MACHINE",
        level: 50,
        scale: 1.0,
        exp: 1122,
        job: 1233,
        hp: 1350,
        sp: 100,
        mp: 100,
        def: [25, 80],
        mdef: [12, 33],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010030, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking Cannon",
        id: 10380003,
        monster_type: "MACHINE",
        level: 50,
        scale: 1.0,
        exp: 1122,
        job: 1233,
        hp: 1350,
        sp: 100,
        mp: 100,
        def: [25, 80],
        mdef: [12, 33],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010031, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking Cannon",
        id: 10380004,
        monster_type: "MACHINE",
        level: 50,
        scale: 1.0,
        exp: 1122,
        job: 1233,
        hp: 1350,
        sp: 100,
        mp: 100,
        def: [25, 80],
        mdef: [12, 33],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010032, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking Cannon",
        id: 10380005,
        monster_type: "MACHINE",
        level: 50,
        scale: 1.0,
        exp: 1122,
        job: 1233,
        hp: 1350,
        sp: 100,
        mp: 100,
        def: [25, 80],
        mdef: [12, 33],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010033, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking Cannon",
        id: 10380006,
        monster_type: "MACHINE",
        level: 50,
        scale: 1.0,
        exp: 1122,
        job: 1233,
        hp: 1350,
        sp: 100,
        mp: 100,
        def: [25, 80],
        mdef: [12, 33],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010034, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Briking Custom",
        id: 10390000,
        monster_type: "MACHINE",
        level: 50,
        scale: 1.0,
        exp: 3672,
        job: 2877,
        hp: 1650,
        sp: 100,
        mp: 100,
        def: [7, 189],
        mdef: [60, 160],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015500, 0, /*null*/
            10023501, 10049000, 0, /*null*/
            10030001, 10050995,
        ],
    },
    Monster {
        name: "Briking Custom Ver0.35",
        id: 10390001,
        monster_type: "MACHINE",
        level: 55,
        scale: 1.0,
        exp: 12375,
        job: 3339,
        hp: 3800,
        sp: 100,
        mp: 100,
        def: [55, 240],
        mdef: [28, 197],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking Custom",
        id: 10390002,
        monster_type: "MACHINE",
        level: 51,
        scale: 1.0,
        exp: 1872,
        job: 1527,
        hp: 1400,
        sp: 100,
        mp: 100,
        def: [7, 139],
        mdef: [25, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010035, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking Custom",
        id: 10390003,
        monster_type: "MACHINE",
        level: 51,
        scale: 1.0,
        exp: 1872,
        job: 1527,
        hp: 1400,
        sp: 100,
        mp: 100,
        def: [7, 139],
        mdef: [25, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010036, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking Custom",
        id: 10390004,
        monster_type: "MACHINE",
        level: 51,
        scale: 1.0,
        exp: 1872,
        job: 1527,
        hp: 1400,
        sp: 100,
        mp: 100,
        def: [7, 139],
        mdef: [25, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010037, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking Custom",
        id: 10390005,
        monster_type: "MACHINE",
        level: 51,
        scale: 1.0,
        exp: 1872,
        job: 1527,
        hp: 1400,
        sp: 100,
        mp: 100,
        def: [7, 139],
        mdef: [25, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010038, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Devastator",
        id: 10400000,
        monster_type: "HUMAN",
        level: 43,
        scale: 0.9,
        exp: 2859,
        job: 1500,
        hp: 1200,
        sp: 100,
        mp: 100,
        def: [15, 150],
        mdef: [24, 76],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001804, 10000103, 10000504, 0, /*null*/
            0, /*null*/
            0, /*null*/
            10050991,
        ],
    },
    Monster {
        name: "Infinite Devastator",
        id: 10400001,
        monster_type: "HUMAN",
        level: 55,
        scale: 0.9,
        exp: 1059,
        job: 600,
        hp: 2200,
        sp: 100,
        mp: 100,
        def: [25, 140],
        mdef: [24, 56],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Red Devastator",
        id: 10400100,
        monster_type: "HUMAN",
        level: 90,
        scale: 1.0,
        exp: 7110,
        job: 4725,
        hp: 10000,
        sp: 250,
        mp: 250,
        def: [10, 290],
        mdef: [50, 75],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10001804, 10033905, 10000107, 60031000, 60051780, 90000044, 0,
        ],
    },
    Monster {
        name: "Hell Diver",
        id: 10400300,
        monster_type: "HUMAN",
        level: 61,
        scale: 1.0,
        exp: 7110,
        job: 4725,
        hp: 2500,
        sp: 250,
        mp: 250,
        def: [10, 150],
        mdef: [20, 55],
        properties: Properties {
            fire: 0,
            water: 10,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10001804, 10033905, 10000107, 60031000, 60051780, 90000044, 0,
        ],
    },
    Monster {
        name: "Avenger",
        id: 10400500,
        monster_type: "HUMAN",
        level: 67,
        scale: 1.0,
        exp: 8100,
        job: 7875,
        hp: 4350,
        sp: 250,
        mp: 250,
        def: [70, 200],
        mdef: [45, 55],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10001804, 10033905, 10000107, 10049000, 10000504, 10009800, 0,
        ],
    },
    Monster {
        name: "Sky Raider",
        id: 10401000,
        monster_type: "HUMAN",
        level: 70,
        scale: 1.0,
        exp: 10575,
        job: 9000,
        hp: 4650,
        sp: 250,
        mp: 250,
        def: [20, 260],
        mdef: [30, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10001804, 10033905, 10000107, 60031051, 60023180, 10054700, 0,
        ],
    },
    Monster {
        name: "Spell Caster",
        id: 10410000,
        monster_type: "HUMAN",
        level: 44,
        scale: 1.0,
        exp: 1620,
        job: 2175,
        hp: 700,
        sp: 100,
        mp: 100,
        def: [56, 45],
        mdef: [10, 40],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 45,
        },
        drop_ids: [
            0, /*null*/
            10009101, 10013350, 10000507, 10049000, 0, /*null*/
            0, /*null*/
            10050992,
        ],
    },
    Monster {
        name: "Infinite Spell Caster",
        id: 10410001,
        monster_type: "HUMAN",
        level: 44,
        scale: 1.0,
        exp: 270,
        job: 360,
        hp: 800,
        sp: 100,
        mp: 100,
        def: [50, 40],
        mdef: [20, 156],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 45,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ancient Spell Caster",
        id: 10410100,
        monster_type: "HUMAN",
        level: 68,
        scale: 1.0,
        exp: 9225,
        job: 8775,
        hp: 3850,
        sp: 300,
        mp: 300,
        def: [55, 140],
        mdef: [40, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 15,
        },
        drop_ids: [
            0, /*null*/
            10009101, 10024900, 10000102, 10049000, 10000507, 10009600, 0,
        ],
    },
    Monster {
        name: "Incubus",
        id: 10410200,
        monster_type: "HUMAN",
        level: 60,
        scale: 1.0,
        exp: 3150,
        job: 3510,
        hp: 2850,
        sp: 250,
        mp: 280,
        def: [45, 120],
        mdef: [90, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10009101, 10020000, 10000102, 10054900, 10054900, 90000045, 0,
        ],
    },
    Monster {
        name: "Militant Spell Caster",
        id: 10410300,
        monster_type: "HUMAN",
        level: 70,
        scale: 1.0,
        exp: 8325,
        job: 9675,
        hp: 3550,
        sp: 350,
        mp: 350,
        def: [55, 150],
        mdef: [40, 60],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10009101, 10020000, 10000102, 10029400, 60081200, 10025051, 0,
        ],
    },
    Monster {
        name: "Lunatic",
        id: 10410500,
        monster_type: "HUMAN",
        level: 16,
        scale: 1.0,
        exp: 306,
        job: 504,
        hp: 560,
        sp: 300,
        mp: 300,
        def: [20, 55],
        mdef: [15, 25],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10037500, 10037800, 0, /*null*/
            10049000, 10037800, 10009600, 0,
        ],
    },
    Monster {
        name: "Lunatic",
        id: 10410501,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 306,
        job: 504,
        hp: 560,
        sp: 300,
        mp: 300,
        def: [10, 45],
        mdef: [15, 25],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Living Armor",
        id: 10420000,
        monster_type: "UNDEAD",
        level: 40,
        scale: 1.0,
        exp: 2205,
        job: 1395,
        hp: 750,
        sp: 180,
        mp: 250,
        def: [30, 150],
        mdef: [80, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10020209, 10000102, 10019900, 10017900, 10019900, 10017900, 10050979,
        ],
    },
    Monster {
        name: "Infinite Living Armor",
        id: 10420001,
        monster_type: "UNDEAD_BOSS",
        level: 45,
        scale: 1.5,
        exp: 135,
        job: 126,
        hp: 2500,
        sp: 180,
        mp: 250,
        def: [20, 130],
        mdef: [30, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Prison Ward",
        id: 10420002,
        monster_type: "UNDEAD",
        level: 25,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 100,
        mp: 100,
        def: [15, 35],
        mdef: [10, 80],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 30,
            earth: 10,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10022604, 10022604, 10022604, 10022604, 10022604, 10022604, 0,
        ],
    },
    Monster {
        name: "Prison Ward",
        id: 10420003,
        monster_type: "UNDEAD",
        level: 25,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 100,
        mp: 100,
        def: [15, 35],
        mdef: [10, 80],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 30,
            earth: 10,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10022604, 10022604, 10022604, 10022604, 10022604, 10022604, 0,
        ],
    },
    Monster {
        name: "Dreadful Armor",
        id: 10420100,
        monster_type: "UNDEAD_BOSS",
        level: 60,
        scale: 2.0,
        exp: 63000,
        job: 54000,
        hp: 25000,
        sp: 180,
        mp: 250,
        def: [40, 230],
        mdef: [35, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10020209, 10000102, 10019900, 10017900, 10019900, 10017900, 0,
        ],
    },
    Monster {
        name: "Death Bringer",
        id: 10420200,
        monster_type: "UNDEAD_BOSS",
        level: 60,
        scale: 2.0,
        exp: 67500,
        job: 49500,
        hp: 27000,
        sp: 180,
        mp: 250,
        def: [35, 250],
        mdef: [30, 180],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10020209, 10000102, 10019900, 10017900, 10019900, 10017900, 0,
        ],
    },
    Monster {
        name: "Necro Armor",
        id: 10420900,
        monster_type: "UNDEAD",
        level: 60,
        scale: 1.0,
        exp: 7425,
        job: 5040,
        hp: 2800,
        sp: 250,
        mp: 250,
        def: [20, 140],
        mdef: [40, 55],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10020209, 10024900, 10000107, 60001200, 10000107, 60001251, 0,
        ],
    },
    Monster {
        name: "Summon Armor",
        id: 10420901,
        monster_type: "UNDEAD",
        level: 70,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 250,
        mp: 250,
        def: [15, 170],
        mdef: [30, 55],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Undead Armor",
        id: 10421000,
        monster_type: "UNDEAD",
        level: 48,
        scale: 1.2,
        exp: 3087,
        job: 3150,
        hp: 3000,
        sp: 180,
        mp: 250,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10020209, 10000102, 10067300, 0, /*null*/
            10019900, 10017900, 0,
        ],
    },
    Monster {
        name: "Noble Armor",
        id: 10421100,
        monster_type: "UNDEAD",
        level: 60,
        scale: 1.0,
        exp: 7260,
        job: 3966,
        hp: 5200,
        sp: 100,
        mp: 100,
        def: [30, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020209, 10000102, 10019900, 10049000, 10067300, 10019900, 0,
        ],
    },
    Monster {
        name: "Infinite Noble Armor",
        id: 10421101,
        monster_type: "UNDEAD",
        level: 55,
        scale: 1.0,
        exp: 2760,
        job: 1986,
        hp: 3200,
        sp: 100,
        mp: 100,
        def: [50, 75],
        mdef: [10, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Chimera",
        id: 10430000,
        monster_type: "BIRD",
        level: 48,
        scale: 1.0,
        exp: 3498,
        job: 1638,
        hp: 1500,
        sp: 100,
        mp: 100,
        def: [35, 105],
        mdef: [25, 135],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006350, 10031901, 0, /*null*/
            10035601, 10035601, 10000401, 10050993,
        ],
    },
    Monster {
        name: "Infinite Chimera",
        id: 10430001,
        monster_type: "BIRD",
        level: 55,
        scale: 1.0,
        exp: 1500,
        job: 738,
        hp: 2100,
        sp: 100,
        mp: 100,
        def: [45, 135],
        mdef: [25, 205],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Evil Chimera",
        id: 10430200,
        monster_type: "BIRD",
        level: 35,
        scale: 1.2,
        exp: 2925,
        job: 12285,
        hp: 1850,
        sp: 100,
        mp: 100,
        def: [10, 75],
        mdef: [10, 60],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 15,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10006404, 0, /*null*/
            90000046, 10031901, 90000046, 0,
        ],
    },
    Monster {
        name: "Evil Chimera",
        id: 10430201,
        monster_type: "BIRD",
        level: 1,
        scale: 1.2,
        exp: 2925,
        job: 12285,
        hp: 1850,
        sp: 100,
        mp: 100,
        def: [10, 75],
        mdef: [10, 60],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 15,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Aqua Chimera",
        id: 10430300,
        monster_type: "BIRD",
        level: 55,
        scale: 1.0,
        exp: 3300,
        job: 4110,
        hp: 2150,
        sp: 100,
        mp: 100,
        def: [30, 120],
        mdef: [10, 75],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006350, 10031901, 0, /*null*/
            10035601, 10035601, 10000401, 0,
        ],
    },
    Monster {
        name: "Gale Chimera",
        id: 10430700,
        monster_type: "BIRD",
        level: 35,
        scale: 1.0,
        exp: 1500,
        job: 2319,
        hp: 1300,
        sp: 100,
        mp: 100,
        def: [5, 85],
        mdef: [25, 140],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 25,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006350, 10018200, 10035601, 0, /*null*/
            10035601, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Venomous Chimera",
        id: 10431000,
        monster_type: "BIRD",
        level: 45,
        scale: 1.0,
        exp: 2400,
        job: 3240,
        hp: 1980,
        sp: 100,
        mp: 100,
        def: [15, 85],
        mdef: [25, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006350, 10031901, 0, /*null*/
            10035601, 10035601, 10000401, 0,
        ],
    },
    Monster {
        name: "Albino Chimera",
        id: 10431400,
        monster_type: "BIRD_BOSS_SKILL",
        level: 65,
        scale: 3.0,
        exp: 31500,
        job: 40500,
        hp: 20000,
        sp: 100,
        mp: 100,
        def: [35, 135],
        mdef: [45, 200],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10026510, 10026510, 10031901, 10033300, 10033300, 10035601, 50035400, 0,
        ],
    },
    Monster {
        name: "Pefang",
        id: 10440000,
        monster_type: "BIRD",
        level: 27,
        scale: 1.0,
        exp: 630,
        job: 789,
        hp: 650,
        sp: 200,
        mp: 120,
        def: [24, 45],
        mdef: [36, 18],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002200, 10009000, 90000043, 10050852, 10009150, 10009500, 10050952,
        ],
    },
    Monster {
        name: "Infinite Pefang",
        id: 10440001,
        monster_type: "BIRD",
        level: 45,
        scale: 1.0,
        exp: 810,
        job: 360,
        hp: 1200,
        sp: 200,
        mp: 120,
        def: [10, 120],
        mdef: [20, 84],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Southern Pefang",
        id: 10440100,
        monster_type: "BIRD",
        level: 38,
        scale: 1.0,
        exp: 1620,
        job: 1734,
        hp: 700,
        sp: 100,
        mp: 100,
        def: [21, 89],
        mdef: [35, 68],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10033910, 10008700, 90000043, 0, /*null*/
            10009300, 10032801, 10050981,
        ],
    },
    Monster {
        name: "Infinite Southern Pefang",
        id: 10440101,
        monster_type: "BIRD",
        level: 48,
        scale: 1.0,
        exp: 1080,
        job: 270,
        hp: 1350,
        sp: 100,
        mp: 100,
        def: [21, 130],
        mdef: [20, 104],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Proonn",
        id: 10440200,
        monster_type: "BIRD",
        level: 31,
        scale: 1.0,
        exp: 1104,
        job: 1149,
        hp: 900,
        sp: 255,
        mp: 200,
        def: [21, 30],
        mdef: [24, 32],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            0, /*null*/
            10002200, 10009300, 90000043, 10050852, 10009150, 10009500, 0,
        ],
    },
    Monster {
        name: "Ramia",
        id: 10450000,
        monster_type: "MAGIC_CREATURE",
        level: 50,
        scale: 1.2,
        exp: 4140,
        job: 4248,
        hp: 1700,
        sp: 100,
        mp: 100,
        def: [15, 234],
        mdef: [25, 150],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 0,
            earth: 60,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005300, 10011205, 10019801, 10049000, 10000310, 10009500, 10050994,
        ],
    },
    Monster {
        name: "King Cobra",
        id: 10450100,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 50,
        scale: 2.0,
        exp: 20250,
        job: 24750,
        hp: 17000,
        sp: 250,
        mp: 250,
        def: [10, 160],
        mdef: [35, 80],
        properties: Properties {
            fire: 25,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10033905, 10011207, 10019801, 10032809, 10043707, 61040180, 0,
        ],
    },
    Monster {
        name: "Rameless",
        id: 10450300,
        monster_type: "MAGIC_CREATURE",
        level: 60,
        scale: 1.2,
        exp: 6300,
        job: 7650,
        hp: 2600,
        sp: 250,
        mp: 250,
        def: [10, 140],
        mdef: [90, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10033905, 10011207, 10019801, 10043707, 10043707, 61040180, 0,
        ],
    },
    Monster {
        name: "Chevales",
        id: 10450900,
        monster_type: "MAGIC_CREATURE",
        level: 74,
        scale: 1.2,
        exp: 10440,
        job: 10890,
        hp: 4600,
        sp: 350,
        mp: 350,
        def: [15, 230],
        mdef: [35, 60],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003800, 10011207, 10000409, 10043707, 10043707, 61040380, 0,
        ],
    },
    Monster {
        name: "Chevales",
        id: 10450901,
        monster_type: "MAGIC_CREATURE",
        level: 74,
        scale: 1.2,
        exp: 10440,
        job: 10890,
        hp: 4600,
        sp: 350,
        mp: 350,
        def: [15, 230],
        mdef: [35, 60],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 25,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003800, 10011207, 10000409, 10043707, 10043707, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ramiloss",
        id: 10451000,
        monster_type: "MAGIC_CREATURE",
        level: 93,
        scale: 1.2,
        exp: 6300,
        job: 7650,
        hp: 12300,
        sp: 250,
        mp: 250,
        def: [10, 180],
        mdef: [90, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 20,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003100, 10000304, 10019801, 10032809, 10043707, 61030054, 0,
        ],
    },
    Monster {
        name: "Mar?ELamuros",
        id: 10451200,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 40,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 4800,
        sp: 0,
        mp: 0,
        def: [10, 100],
        mdef: [30, 90],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Ramia",
        id: 10450001,
        monster_type: "MAGIC_CREATURE",
        level: 55,
        scale: 1.2,
        exp: 1350,
        job: 1395,
        hp: 2150,
        sp: 100,
        mp: 100,
        def: [10, 230],
        mdef: [55, 275],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dominion Blackguard",
        id: 10460000,
        monster_type: "HUMAN",
        level: 56,
        scale: 1.0,
        exp: 6675,
        job: 3960,
        hp: 2050,
        sp: 100,
        mp: 100,
        def: [70, 200],
        mdef: [26, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 60,
        },
        drop_ids: [
            0, /*null*/
            10009100, 10000102, 10034504, 10049000, 0, /*null*/
            0, /*null*/
            10050997,
        ],
    },
    Monster {
        name: "Dominion Animus",
        id: 10460100,
        monster_type: "HUMAN",
        level: 71,
        scale: 1.0,
        exp: 9135,
        job: 9000,
        hp: 6000,
        sp: 300,
        mp: 300,
        def: [70, 190],
        mdef: [45, 175],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10000102, 10000101, 10012600, 90000045, 10034504, 0,
        ],
    },
    Monster {
        name: "Dominion High Blackguard",
        id: 10470000,
        monster_type: "HUMAN",
        level: 62,
        scale: 1.0,
        exp: 6801,
        job: 7287,
        hp: 3500,
        sp: 100,
        mp: 100,
        def: [25, 130],
        mdef: [65, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 70,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009100, 10000102, 10034506, 10049000, 0, /*null*/
            0, /*null*/
            10050998,
        ],
    },
    Monster {
        name: "Dominion Malice",
        id: 10470100,
        monster_type: "HUMAN",
        level: 73,
        scale: 1.0,
        exp: 10125,
        job: 9360,
        hp: 6800,
        sp: 400,
        mp: 400,
        def: [30, 150],
        mdef: [65, 105],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10000102, 10000101, 10012600, 90000045, 10034504, 0,
        ],
    },
    Monster {
        name: "Dominion Pennant",
        id: 10470001,
        monster_type: "HUMAN",
        level: 92,
        scale: 1.0,
        exp: 17910,
        job: 19575,
        hp: 24000,
        sp: 400,
        mp: 400,
        def: [60, 150],
        mdef: [65, 105],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10009101, 10009900, 10019900, 60081200, 10029400, 60090350, 0,
        ],
    },
    Monster {
        name: "Fake Monster 1",
        id: 10480000,
        monster_type: "ANIMAL",
        level: 6,
        scale: 1.0,
        exp: 108,
        job: 9,
        hp: 110,
        sp: 0,
        mp: 0,
        def: [7, 15],
        mdef: [10, 4],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006600, 10006300, 10020200, 0, /*null*/
            10006400, 10020210, 0,
        ],
    },
    Monster {
        name: "Jumbo ChockKo",
        id: 10490000,
        monster_type: "BIRD_BOSS_SKILL",
        level: 10,
        scale: 2.0,
        exp: 45000,
        job: 18000,
        hp: 19800,
        sp: 0,
        mp: 0,
        def: [36, 56],
        mdef: [57, 84],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10012550, 10012550, 10012000, 50040100, 10049500, 50040100, 10049500, 10051005,
        ],
    },
    Monster {
        name: "Giant Cockatrice",
        id: 10490001,
        monster_type: "BIRD_BOSS_SKILL",
        level: 10,
        scale: 2.0,
        exp: 45000,
        job: 18000,
        hp: 19800,
        sp: 0,
        mp: 0,
        def: [36, 56],
        mdef: [57, 84],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10051005,
        ],
    },
    Monster {
        name: "Briking MkII",
        id: 10500000,
        monster_type: "MACHINE_BOSS_SKILL",
        level: 10,
        scale: 3.0,
        exp: 45000,
        job: 27000,
        hp: 25800,
        sp: 0,
        mp: 0,
        def: [30, 108],
        mdef: [18, 52],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [
            10018000, 10018000, 10030001, 10018003, 10049500, 10018003, 10049500, 0,
        ],
    },
    Monster {
        name: "Briking Noel",
        id: 10500050,
        monster_type: "MACHINE_BOSS_SKILL",
        level: 55,
        scale: 3.0,
        exp: 1125,
        job: 1125,
        hp: 300000,
        sp: 0,
        mp: 0,
        def: [11, 33],
        mdef: [18, 52],
        properties: Properties {
            fire: 130,
            water: 130,
            wind: 130,
            earth: 130,
            light: 255,
            dark: 255,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "King Roper",
        id: 10510000,
        monster_type: "PLANT_BOSS_SKILL",
        level: 10,
        scale: 2.5,
        exp: 44550,
        job: 29250,
        hp: 25400,
        sp: 0,
        mp: 0,
        def: [75, 2],
        mdef: [90, 2],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10025100, 10025100, 10012500, 50040900, 10049500, 50040900, 10049500, 10051004,
        ],
    },
    Monster {
        name: "King Roper",
        id: 10510001,
        monster_type: "PLANT_BOSS_SKILL",
        level: 10,
        scale: 2.5,
        exp: 44550,
        job: 29250,
        hp: 25400,
        sp: 0,
        mp: 0,
        def: [75, 2],
        mdef: [90, 2],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10025100, 10025100, 10012500, 50040900, 10049500, 50040900, 10049500, 10051004,
        ],
    },
    Monster {
        name: "King Roper",
        id: 10510002,
        monster_type: "PLANT_BOSS_SKILL",
        level: 10,
        scale: 2.5,
        exp: 44550,
        job: 29250,
        hp: 25400,
        sp: 0,
        mp: 0,
        def: [75, 2],
        mdef: [90, 2],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10051004,
        ],
    },
    Monster {
        name: "Stinger",
        id: 10520000,
        monster_type: "INSECT_BOSS_SKILL",
        level: 10,
        scale: 2.0,
        exp: 675,
        job: 360,
        hp: 4500,
        sp: 0,
        mp: 0,
        def: [10, 14],
        mdef: [18, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000408, 10000408, 10000408, 10000408, 10009111, 10000408, 10009111, 0,
        ],
    },
    Monster {
        name: "White Bear",
        id: 10530000,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 10,
        scale: 1.4,
        exp: 58500,
        job: 24750,
        hp: 37800,
        sp: 0,
        mp: 0,
        def: [30, 66],
        mdef: [81, 48],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020551, 10020551, 10012200, 0, /*null*/
            10049500, 0, /*null*/
            10049500, 10051003,
        ],
    },
    Monster {
        name: "White Bear",
        id: 10530001,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 10,
        scale: 1.4,
        exp: 58500,
        job: 24750,
        hp: 37800,
        sp: 0,
        mp: 0,
        def: [30, 66],
        mdef: [81, 48],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10051003,
        ],
    },
    Monster {
        name: "Gold Bear",
        id: 10530700,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 10,
        scale: 1.4,
        exp: 33750,
        job: 15750,
        hp: 11000,
        sp: 0,
        mp: 0,
        def: [11, 66],
        mdef: [81, 48],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020551, 10020551, 10012200, 0, /*null*/
            10049500, 0, /*null*/
            10049500, 0,
        ],
    },
    Monster {
        name: "Mercenary",
        id: 10540000,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.2,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mercenary A",
        id: 10540003,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.2,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mercenary B",
        id: 10540002,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.2,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Bandit",
        id: 10540500,
        monster_type: "HUMAN",
        level: 74,
        scale: 1.2,
        exp: 9450,
        job: 10800,
        hp: 4400,
        sp: 250,
        mp: 250,
        def: [10, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10033905, 10020202, 90000043, 10015300, 61010600, 90000043, 0,
        ],
    },
    Monster {
        name: "Handsome Mercenary",
        id: 10540501,
        monster_type: "HUMAN_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 9450,
        job: 10800,
        hp: 4400,
        sp: 250,
        mp: 250,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ninja",
        id: 10540502,
        monster_type: "HUMAN_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 9450,
        job: 10800,
        hp: 4400,
        sp: 250,
        mp: 250,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dragon Dance",
        id: 10540503,
        monster_type: "HUMAN_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4400,
        sp: 250,
        mp: 250,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Takoyaki Urchin",
        id: 10540504,
        monster_type: "ANIMAL_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4400,
        sp: 250,
        mp: 250,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mentai",
        id: 10540505,
        monster_type: "ANIMAL_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4400,
        sp: 250,
        mp: 250,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Penguin Squad",
        id: 10540506,
        monster_type: "ANIMAL_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4400,
        sp: 250,
        mp: 250,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Healer Balloon Rabbit",
        id: 10540507,
        monster_type: "ANIMAL_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4400,
        sp: 250,
        mp: 250,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gathering Gun",
        id: 10550000,
        monster_type: "MACHINE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gathering Gun A",
        id: 10550016,
        monster_type: "MACHINE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gathering Gun B",
        id: 10550017,
        monster_type: "MACHINE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Green Crawler Carrier",
        id: 10560000,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Green Mini Crawler Carrier",
        id: 10560001,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Puchi Crawler",
        id: 10560002,
        monster_type: "INSECT",
        level: 1,
        scale: 0.5,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ranch Puchi Crawler",
        id: 10560003,
        monster_type: "INSECT_NOTOUCH",
        level: 1,
        scale: 0.5,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Red Crawler Carrier",
        id: 10560100,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Red Mini Crawler Carrier",
        id: 10560101,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Purple Crawler Carrier",
        id: 10560200,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Purple Mini Crawler Carrier",
        id: 10560201,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blue Crawler Carrier",
        id: 10560400,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blue Mini Crawler Carrier",
        id: 10560401,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Yellow Crawler Carrier",
        id: 10560800,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Yellow Mini Crawler Carrier",
        id: 10560801,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fluffy Crawler Carrier",
        id: 10565000,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fluffy Mini Crawler Carrier",
        id: 10565001,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Rock Crawler Carrier",
        id: 10566000,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Rock Mini Crawler Carrier",
        id: 10566001,
        monster_type: "INSECT",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Puppy",
        id: 10570000,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Puppy A",
        id: 10570020,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Puppy B",
        id: 10570021,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wild Puppy",
        id: 10570001,
        monster_type: "ANIMAL_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Shiro",
        id: 10570002,
        monster_type: "ANIMAL_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blacktan",
        id: 10570050,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blacktan A",
        id: 10570010,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blacktan B",
        id: 10570014,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Graze Blacktan",
        id: 10570051,
        monster_type: "ANIMAL_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Kuro",
        id: 10570052,
        monster_type: "ANIMAL_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Carrot",
        id: 10580000,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wild Carrot",
        id: 10580001,
        monster_type: "PLANT_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Turnip",
        id: 10580002,
        monster_type: "PLANT",
        level: 1,
        scale: 1.1,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Turnip A",
        id: 10580003,
        monster_type: "PLANT",
        level: 1,
        scale: 1.2,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Carrot",
        id: 10580004,
        monster_type: "PLANT_NOTOUCH",
        level: 1,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Wall",
        id: 10580005,
        monster_type: "PLANT_NOTOUCH",
        level: 1,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 0,
        mp: 0,
        def: [0, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Wall",
        id: 10580006,
        monster_type: "PLANT_NOTOUCH",
        level: 1,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 2000,
        sp: 0,
        mp: 0,
        def: [10, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Wall",
        id: 10580007,
        monster_type: "PLANT_NOTOUCH",
        level: 1,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 3000,
        sp: 0,
        mp: 0,
        def: [40, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Wall",
        id: 10580008,
        monster_type: "PLANT_NOTOUCH",
        level: 1,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 4000,
        sp: 0,
        mp: 0,
        def: [70, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Wall",
        id: 10580009,
        monster_type: "PLANT_NOTOUCH",
        level: 1,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 5000,
        sp: 0,
        mp: 0,
        def: [90, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Daikon",
        id: 10580010,
        monster_type: "PLANT_MARK",
        level: 1,
        scale: 1.1,
        exp: 0,
        job: 0,
        hp: 3,
        sp: 0,
        mp: 0,
        def: [99, 0],
        mdef: [99, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Ninjin",
        id: 10580011,
        monster_type: "PLANT_MARK",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 0,
        mp: 0,
        def: [99, 0],
        mdef: [100, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandra Wasabi",
        id: 10580500,
        monster_type: "PLANT",
        level: 69,
        scale: 1.0,
        exp: 9000,
        job: 9000,
        hp: 5800,
        sp: 0,
        mp: 0,
        def: [20, 250],
        mdef: [70, 130],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10004908, 10004901, 10004903, 10000302, 0, /*null*/
            10009600, 90000045, 0,
        ],
    },
    Monster {
        name: "Mandra Wasabi",
        id: 10580501,
        monster_type: "PLANT_MARK",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 0,
        mp: 0,
        def: [99, 0],
        mdef: [99, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sori(Not Implemented Yet)",
        id: 10590000,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 600,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Snowman(Not Implemented Yet)",
        id: 10600000,
        monster_type: "ELEMENT",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 600,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Snowman",
        id: 10600001,
        monster_type: "ELEMENT",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 600,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Cocoa",
        id: 10610000,
        monster_type: "ANIMAL",
        level: 25,
        scale: 1.0,
        exp: 360,
        job: 261,
        hp: 760,
        sp: 100,
        mp: 100,
        def: [12, 55],
        mdef: [21, 49],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 22,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ranch Dachshund",
        id: 10610001,
        monster_type: "ANIMAL_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Young CockKo",
        id: 10620000,
        monster_type: "BIRD",
        level: 27,
        scale: 1.3,
        exp: 315,
        job: 225,
        hp: 270,
        sp: 50,
        mp: 50,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10033905, 10034502, 0, /*null*/
            10006400, 10009500, 0,
        ],
    },
    Monster {
        name: "Wild Young CockKo",
        id: 10620001,
        monster_type: "BIRD_NOTOUCH",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "CockKo Baby",
        id: 10620100,
        monster_type: "BIRD",
        level: 24,
        scale: 2.0,
        exp: 270,
        job: 315,
        hp: 300,
        sp: 100,
        mp: 100,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007700, 10006300, 10033910, 0, /*null*/
            10006400, 10009500, 0,
        ],
    },
    Monster {
        name: "Piyo Piyo",
        id: 10620200,
        monster_type: "BIRD",
        level: 5,
        scale: 1.3,
        exp: 105,
        job: 180,
        hp: 120,
        sp: 50,
        mp: 50,
        def: [7, 10],
        mdef: [7, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10018208, 10019900, 0, /*null*/
            90000046, 10018208, 90000046, 0,
        ],
    },
    Monster {
        name: "Piyo Piyo",
        id: 10620201,
        monster_type: "BIRD",
        level: 1,
        scale: 1.3,
        exp: 105,
        job: 180,
        hp: 120,
        sp: 50,
        mp: 50,
        def: [7, 10],
        mdef: [7, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Chirm",
        id: 10620400,
        monster_type: "BIRD",
        level: 30,
        scale: 1.3,
        exp: 1125,
        job: 1260,
        hp: 1350,
        sp: 50,
        mp: 50,
        def: [20, 80],
        mdef: [35, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10002200, 90000044, 0, /*null*/
            10037300, 90000044, 0,
        ],
    },
    Monster {
        name: "Light Blue Cococko",
        id: 10620600,
        monster_type: "BIRD",
        level: 23,
        scale: 1.3,
        exp: 315,
        job: 225,
        hp: 450,
        sp: 50,
        mp: 50,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10033905, 10034502, 0, /*null*/
            10006400, 10009500, 0,
        ],
    },
    Monster {
        name: "Cokaras",
        id: 10620900,
        monster_type: "BIRD",
        level: 45,
        scale: 1.3,
        exp: 2475,
        job: 2160,
        hp: 2550,
        sp: 255,
        mp: 255,
        def: [10, 90],
        mdef: [40, 190],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007800, 10018209, 10033905, 0, /*null*/
            10018209, 10009500, 0,
        ],
    },
    Monster {
        name: "Gecko",
        id: 10630000,
        monster_type: "ANIMAL",
        level: 38,
        scale: 1.0,
        exp: 1575,
        job: 2025,
        hp: 2100,
        sp: 255,
        mp: 255,
        def: [10, 90],
        mdef: [20, 150],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10015302, 10051600, 10012300, 10013002, 10009600, 0,
        ],
    },
    Monster {
        name: "Elder Gecko",
        id: 10630200,
        monster_type: "ANIMAL",
        level: 58,
        scale: 1.0,
        exp: 6525,
        job: 5733,
        hp: 3500,
        sp: 255,
        mp: 255,
        def: [20, 125],
        mdef: [40, 190],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015302, 10051600, 10045000, 0, /*null*/
            10015250, 10009600, 0,
        ],
    },
    Monster {
        name: "Reckless Gecko",
        id: 10630700,
        monster_type: "ANIMAL",
        level: 35,
        scale: 1.0,
        exp: 1755,
        job: 1800,
        hp: 840,
        sp: 100,
        mp: 100,
        def: [10, 40],
        mdef: [50, 10],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10003200, 10000305, 10012300, 10039300, 10009500, 0,
        ],
    },
    Monster {
        name: "Alt Gecko",
        id: 10635000,
        monster_type: "ANIMAL",
        level: 52,
        scale: 1.0,
        exp: 5175,
        job: 4275,
        hp: 2300,
        sp: 255,
        mp: 255,
        def: [20, 95],
        mdef: [40, 80],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015302, 10051600, 10045000, 10067300, 10026350, 10009600, 0,
        ],
    },
    Monster {
        name: "Vega",
        id: 10640000,
        monster_type: "ANIMAL",
        level: 25,
        scale: 1.0,
        exp: 450,
        job: 360,
        hp: 400,
        sp: 100,
        mp: 100,
        def: [15, 35],
        mdef: [10, 80],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 20,
            earth: 10,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10004200, 10033905, 10051500, 10052000, 10004200, 10009500, 0,
        ],
    },
    Monster {
        name: "Wild Vega",
        id: 10640001,
        monster_type: "ANIMAL_NOTOUCH",
        level: 25,
        scale: 1.0,
        exp: 360,
        job: 180,
        hp: 400,
        sp: 100,
        mp: 100,
        def: [15, 35],
        mdef: [10, 80],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 30,
            earth: 10,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Veaal",
        id: 10640200,
        monster_type: "ANIMAL",
        level: 65,
        scale: 1.0,
        exp: 7650,
        job: 7875,
        hp: 3550,
        sp: 255,
        mp: 255,
        def: [30, 130],
        mdef: [40, 165],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10051500, 10033905, 10020210, 0, /*null*/
            10051500, 10052000, 0,
        ],
    },
    Monster {
        name: "Heavenly Vega",
        id: 10640300,
        monster_type: "ANIMAL",
        level: 26,
        scale: 1.0,
        exp: 1035,
        job: 1035,
        hp: 1000,
        sp: 150,
        mp: 150,
        def: [15, 75],
        mdef: [10, 85],
        properties: Properties {
            fire: 10,
            water: 40,
            wind: 10,
            earth: 10,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10004200, 10033905, 10051500, 0, /*null*/
            10004200, 10052000, 0,
        ],
    },
    Monster {
        name: "ECO Pig",
        id: 10650000,
        monster_type: "ANIMAL",
        level: 24,
        scale: 1.0,
        exp: 405,
        job: 339,
        hp: 300,
        sp: 100,
        mp: 100,
        def: [15, 35],
        mdef: [20, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10004902, 10004908, 10005604, 0, /*null*/
            10020550, 10005200, 0,
        ],
    },
    Monster {
        name: "Ranch ECO Pig",
        id: 10655001,
        monster_type: "ANIMAL_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 2100,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wild Boar",
        id: 10650100,
        monster_type: "ANIMAL",
        level: 28,
        scale: 1.5,
        exp: 474,
        job: 609,
        hp: 450,
        sp: 200,
        mp: 200,
        def: [10, 50],
        mdef: [15, 55],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10020300, 10005607, 0, /*null*/
            10020210, 10005200, 0,
        ],
    },
    Monster {
        name: "Angel Boar",
        id: 10650700,
        monster_type: "ANIMAL",
        level: 82,
        scale: 1.0,
        exp: 10800,
        job: 9000,
        hp: 7770,
        sp: 200,
        mp: 200,
        def: [40, 155],
        mdef: [35, 55],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 20,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018204, 10005646, 10005646, 10054600, 10020404, 10020404, 0,
        ],
    },
    Monster {
        name: "Wild Black Boar",
        id: 10650900,
        monster_type: "ANIMAL",
        level: 90,
        scale: 0.8,
        exp: 14400,
        job: 15075,
        hp: 30000,
        sp: 400,
        mp: 400,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 49,
        },
        drop_ids: [
            0, /*null*/
            10018204, 10005646, 1005646, 1005600, 10020404, 10020404, 0,
        ],
    },
    Monster {
        name: "Flower Pig",
        id: 10651200,
        monster_type: "ANIMAL",
        level: 48,
        scale: 1.3,
        exp: 3375,
        job: 3375,
        hp: 3375,
        sp: 200,
        mp: 200,
        def: [10, 85],
        mdef: [45, 55],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037501, 10033905, 10020409, 10049000, 90000043, 10005200, 0,
        ],
    },
    Monster {
        name: "Momo",
        id: 10660000,
        monster_type: "ANIMAL",
        level: 27,
        scale: 0.7,
        exp: 348,
        job: 405,
        hp: 360,
        sp: 100,
        mp: 100,
        def: [10, 10],
        mdef: [10, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002200, 10020300, 10020351, 0, /*null*/
            10037300, 10009500, 0,
        ],
    },
    Monster {
        name: "Momo A",
        id: 10660001,
        monster_type: "ANIMAL",
        level: 27,
        scale: 0.9,
        exp: 249,
        job: 270,
        hp: 300,
        sp: 100,
        mp: 100,
        def: [10, 10],
        mdef: [10, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002200, 10020300, 10020351, 0, /*null*/
            10037300, 10009500, 0,
        ],
    },
    Monster {
        name: "Momo B",
        id: 10660002,
        monster_type: "ANIMAL",
        level: 27,
        scale: 1.0,
        exp: 249,
        job: 270,
        hp: 300,
        sp: 100,
        mp: 100,
        def: [10, 10],
        mdef: [10, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002200, 10020300, 10020351, 0, /*null*/
            10037300, 10009500, 0,
        ],
    },
    Monster {
        name: "Rainbow Momo",
        id: 10665000,
        monster_type: "ANIMAL",
        level: 27,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 100,
        mp: 100,
        def: [10, 10],
        mdef: [10, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fake Scarecrow",
        id: 10670000,
        monster_type: "MAGIC_CREATURE",
        level: 45,
        scale: 1.0,
        exp: 2700,
        job: 3150,
        hp: 2000,
        sp: 300,
        mp: 300,
        def: [30, 80],
        mdef: [20, 120],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10045902, 10021300, 10019601, 0, /*null*/
            10012100, 10009600, 0,
        ],
    },
    Monster {
        name: "Clone Scarecrow",
        id: 10670002,
        monster_type: "MAGIC_CREATURE",
        level: 50,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 1980,
        sp: 300,
        mp: 300,
        def: [50, 160],
        mdef: [60, 190],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 20,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Red Scarecrow",
        id: 10670100,
        monster_type: "MAGIC_CREATURE",
        level: 45,
        scale: 1.0,
        exp: 2700,
        job: 3150,
        hp: 2000,
        sp: 300,
        mp: 300,
        def: [30, 80],
        mdef: [20, 120],
        properties: Properties {
            fire: 20,
            water: 20,
            wind: 20,
            earth: 20,
            light: 20,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10045902, 10021300, 10019601, 0, /*null*/
            10012100, 10009600, 0,
        ],
    },
    Monster {
        name: "Surveyor",
        id: 10670800,
        monster_type: "MAGIC_CREATURE",
        level: 94,
        scale: 1.0,
        exp: 17100,
        job: 18000,
        hp: 14500,
        sp: 300,
        mp: 300,
        def: [30, 160],
        mdef: [20, 130],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10051500, 10045902, 10019900, 0, /*null*/
            10044103, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Scarecrow Leader",
        id: 10670900,
        monster_type: "MAGIC_CREATURE",
        level: 65,
        scale: 1.0,
        exp: 7875,
        job: 8325,
        hp: 3000,
        sp: 300,
        mp: 300,
        def: [20, 130],
        mdef: [30, 140],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 20,
            earth: 20,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10045902, 10021300, 10019601, 0, /*null*/
            10012100, 10009600, 0,
        ],
    },
    Monster {
        name: "Reckless Scarecrow",
        id: 10675000,
        monster_type: "MAGIC_CREATURE",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ghost",
        id: 10680000,
        monster_type: "MAGIC_CREATURE",
        level: 45,
        scale: 1.0,
        exp: 2475,
        job: 2655,
        hp: 2100,
        sp: 200,
        mp: 200,
        def: [15, 90],
        mdef: [55, 105],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10009101, 10024900, 0, /*null*/
            10001200, 10009600, 0,
        ],
    },
    Monster {
        name: "?",
        id: 10680001,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 45,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 2100,
        sp: 200,
        mp: 200,
        def: [15, 90],
        mdef: [55, 105],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Scalet Ghost",
        id: 10680100,
        monster_type: "MAGIC_CREATURE",
        level: 60,
        scale: 1.0,
        exp: 6525,
        job: 7650,
        hp: 2400,
        sp: 200,
        mp: 200,
        def: [15, 95],
        mdef: [55, 175],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10009101, 10024900, 0, /*null*/
            10001200, 10009600, 0,
        ],
    },
    Monster {
        name: "Purple Ghost",
        id: 10680200,
        monster_type: "MAGIC_CREATURE",
        level: 45,
        scale: 1.0,
        exp: 2475,
        job: 2655,
        hp: 2100,
        sp: 200,
        mp: 200,
        def: [15, 90],
        mdef: [55, 105],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10009101, 10024900, 0, /*null*/
            10001200, 10009600, 0,
        ],
    },
    Monster {
        name: "Blue Ghost",
        id: 10680300,
        monster_type: "MAGIC_CREATURE",
        level: 45,
        scale: 1.0,
        exp: 2475,
        job: 2655,
        hp: 2100,
        sp: 200,
        mp: 200,
        def: [15, 90],
        mdef: [55, 105],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10009101, 10024900, 0, /*null*/
            10001200, 10009600, 0,
        ],
    },
    Monster {
        name: "Lesser Ghost",
        id: 10680800,
        monster_type: "MAGIC_CREATURE",
        level: 40,
        scale: 0.7,
        exp: 1260,
        job: 1440,
        hp: 1500,
        sp: 200,
        mp: 200,
        def: [15, 90],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10035000, 10009101, 0, /*null*/
            10035000, 10009500, 0,
        ],
    },
    Monster {
        name: "Raven Ghost",
        id: 10680900,
        monster_type: "MAGIC_CREATURE",
        level: 52,
        scale: 1.0,
        exp: 2475,
        job: 2655,
        hp: 2350,
        sp: 200,
        mp: 200,
        def: [20, 90],
        mdef: [50, 170],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10001200, 10009101, 10024900, 0, /*null*/
            10001200, 10009600, 0,
        ],
    },
    Monster {
        name: "Panda Ghost",
        id: 10681500,
        monster_type: "MAGIC_CREATURE",
        level: 24,
        scale: 0.75,
        exp: 294,
        job: 294,
        hp: 400,
        sp: 113,
        mp: 299,
        def: [65, 95],
        mdef: [52, 60],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 60,
        },
        drop_ids: [
            0, /*null*/
            10002202, 0, /*null*/
            10032802, 10001200, 10014500, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Spector",
        id: 10690000,
        monster_type: "UNDEAD",
        level: 45,
        scale: 1.0,
        exp: 2610,
        job: 2430,
        hp: 2200,
        sp: 200,
        mp: 200,
        def: [20, 100],
        mdef: [45, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10024900, 10009101, 50023050, 0, /*null*/
            10009101, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Clone Phantom B",
        id: 10690001,
        monster_type: "MAGIC_CREATURE",
        level: 50,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 2000,
        sp: 200,
        mp: 200,
        def: [10, 230],
        mdef: [75, 275],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Summon Specter",
        id: 10690002,
        monster_type: "UNDEAD",
        level: 70,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 200,
        mp: 200,
        def: [10, 130],
        mdef: [60, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Red Specter",
        id: 10690100,
        monster_type: "UNDEAD_SKILL",
        level: 12,
        scale: 3.0,
        exp: 0,
        job: 0,
        hp: 25600,
        sp: 0,
        mp: 0,
        def: [99, 240],
        mdef: [99, 240],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Scary Ghost",
        id: 10690101,
        monster_type: "UNDEAD_SKILL",
        level: 12,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 2560,
        sp: 0,
        mp: 0,
        def: [99, 240],
        mdef: [99, 240],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Revenant",
        id: 10690200,
        monster_type: "UNDEAD",
        level: 25,
        scale: 1.0,
        exp: 645,
        job: 1518,
        hp: 865,
        sp: 200,
        mp: 200,
        def: [15, 35],
        mdef: [10, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10000202, 10019900, 0, /*null*/
            90000045, 10000202, 90000045, 0,
        ],
    },
    Monster {
        name: "Revenant",
        id: 10690201,
        monster_type: "UNDEAD",
        level: 1,
        scale: 1.0,
        exp: 645,
        job: 1518,
        hp: 865,
        sp: 200,
        mp: 200,
        def: [15, 35],
        mdef: [10, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blue Specter",
        id: 10690300,
        monster_type: "UNDEAD",
        level: 45,
        scale: 1.0,
        exp: 2610,
        job: 2430,
        hp: 2200,
        sp: 200,
        mp: 200,
        def: [20, 100],
        mdef: [45, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10024900, 10009101, 50023050, 0, /*null*/
            10009101, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Green Specter",
        id: 10690500,
        monster_type: "UNDEAD",
        level: 45,
        scale: 1.0,
        exp: 2610,
        job: 2430,
        hp: 2200,
        sp: 200,
        mp: 200,
        def: [20, 100],
        mdef: [45, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10024900, 10009101, 50023050, 0, /*null*/
            10009101, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Lesser Specter Specter",
        id: 10690700,
        monster_type: "UNDEAD",
        level: 40,
        scale: 0.7,
        exp: 1440,
        job: 1350,
        hp: 1800,
        sp: 200,
        mp: 200,
        def: [20, 100],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            10024900, 10009101, 10035000, 0, /*null*/
            10009101, 10009500, 0,
        ],
    },
    Monster {
        name: "Pirate Bomber",
        id: 10700000,
        monster_type: "MAGIC_CREATURE",
        level: 74,
        scale: 0.6,
        exp: 10800,
        job: 9000,
        hp: 4500,
        sp: 300,
        mp: 300,
        def: [15, 185],
        mdef: [45, 50],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001550, 10042701, 10032801, 10019601, 10042701, 10032301, 0,
        ],
    },
    Monster {
        name: "Clone Pirate A",
        id: 10700001,
        monster_type: "HUMAN",
        level: 74,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 4500,
        sp: 300,
        mp: 300,
        def: [15, 190],
        mdef: [45, 50],
        properties: Properties {
            fire: 10,
            water: 20,
            wind: 10,
            earth: 10,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10001550, 10042701, 10022308, 0, /*null*/
            10032301, 10032301, 0,
        ],
    },
    Monster {
        name: "Franker Pirate",
        id: 10710000,
        monster_type: "MAGIC_CREATURE",
        level: 75,
        scale: 0.7,
        exp: 11475,
        job: 8550,
        hp: 4300,
        sp: 350,
        mp: 350,
        def: [20, 210],
        mdef: [70, 145],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10033905, 10042701, 10032803, 10019601, 10042701, 10032303, 0,
        ],
    },
    Monster {
        name: "Clone Pirate B",
        id: 10710001,
        monster_type: "HUMAN",
        level: 50,
        scale: 0.95,
        exp: 15,
        job: 6,
        hp: 500,
        sp: 200,
        mp: 200,
        def: [10, 230],
        mdef: [75, 275],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wild Swine",
        id: 10720000,
        monster_type: "ANIMAL_NOTOUCH",
        level: 25,
        scale: 1.0,
        exp: 360,
        job: 261,
        hp: 760,
        sp: 100,
        mp: 100,
        def: [12, 55],
        mdef: [21, 49],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 22,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pegasus",
        id: 10730000,
        monster_type: "ANIMAL",
        level: 1,
        scale: 0.2,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pegasus A",
        id: 10730007,
        monster_type: "ANIMAL",
        level: 1,
        scale: 0.2,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pegasus B",
        id: 10730008,
        monster_type: "ANIMAL",
        level: 1,
        scale: 0.2,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sleepy",
        id: 10740000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 0.8,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sleepy 2",
        id: 10740001,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.6,
        exp: 15,
        job: 6,
        hp: 15000,
        sp: 0,
        mp: 0,
        def: [25, 300],
        mdef: [70, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Rideable Pet 1",
        id: 10750000,
        monster_type: "INSECT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Rideable Pet 2",
        id: 10760000,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Red Test",
        id: 10760100,
        monster_type: "INSECT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Yellow Test",
        id: 10760800,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sky Squirrel",
        id: 10780000,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.15,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sky Squirrel A",
        id: 10780002,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.15,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sky Squirrel B",
        id: 10780003,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.15,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pink Squirrel",
        id: 10780001,
        monster_type: "ANIMAL_NOTOUCH",
        level: 1,
        scale: 1.15,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pink Mouse",
        id: 10781000,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.15,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Killer Machine",
        id: 10800000,
        monster_type: "MACHINE",
        level: 86,
        scale: 1.0,
        exp: 13500,
        job: 18000,
        hp: 15000,
        sp: 0,
        mp: 0,
        def: [20, 350],
        mdef: [80, 80],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10030500, 10038400, 10019900, 10054700, 10023502, 10023504, 0,
        ],
    },
    Monster {
        name: "Killer Machine ??",
        id: 10800001,
        monster_type: "MACHINE",
        level: 90,
        scale: 2.0,
        exp: 18000,
        job: 20250,
        hp: 22500,
        sp: 0,
        mp: 0,
        def: [90, 300],
        mdef: [50, 140],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10030500, 10016150, 10038400, 10019900, 60051780, 10023506, 10023508, 0,
        ],
    },
    Monster {
        name: "killer Machine E",
        id: 10800002,
        monster_type: "MACHINE",
        level: 10,
        scale: 1.0,
        exp: 15,
        job: 18,
        hp: 1000,
        sp: 0,
        mp: 0,
        def: [10, 20],
        mdef: [50, 50],
        properties: Properties {
            fire: 80,
            water: 80,
            wind: 80,
            earth: 80,
            light: 80,
            dark: 80,
        },
        drop_ids: [
            10047413, 10047407, 10047407, 10047408, 10047408, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Evil Chimera",
        id: 10800200,
        monster_type: "MACHINE",
        level: 90,
        scale: 2.0,
        exp: 18000,
        job: 20250,
        hp: 22500,
        sp: 0,
        mp: 0,
        def: [90, 300],
        mdef: [50, 140],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10030500, 10016150, 10038400, 10019900, 60051780, 10023506, 10023508, 0,
        ],
    },
    Monster {
        name: "Green Chimera",
        id: 10800500,
        monster_type: "MACHINE",
        level: 90,
        scale: 2.0,
        exp: 18000,
        job: 20250,
        hp: 22500,
        sp: 0,
        mp: 0,
        def: [90, 300],
        mdef: [50, 140],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10030500, 10016150, 10038400, 10019900, 60051780, 10023506, 10023508, 0,
        ],
    },
    Monster {
        name: "Fox Trot S5",
        id: 10810000,
        monster_type: "MACHINE",
        level: 80,
        scale: 1.0,
        exp: 12600,
        job: 12600,
        hp: 7500,
        sp: 300,
        mp: 300,
        def: [10, 230],
        mdef: [50, 55],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10030500, 10016150, 10019900, 60051780, 10023502, 10023504, 0,
        ],
    },
    Monster {
        name: "Robot Drill",
        id: 10810001,
        monster_type: "MACHINE",
        level: 1,
        scale: 2.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fox Trot R4",
        id: 10810100,
        monster_type: "MACHINE",
        level: 71,
        scale: 0.7,
        exp: 11250,
        job: 11250,
        hp: 5000,
        sp: 300,
        mp: 300,
        def: [10, 225],
        mdef: [25, 150],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10017100, 0, /*null*/
            10023502, 10054300, 60023180, 10023505, 0,
        ],
    },
    Monster {
        name: "Crush Bug",
        id: 10810101,
        monster_type: "MACHINE_SKILL",
        level: 1,
        scale: 0.3,
        exp: 6,
        job: 45,
        hp: 3,
        sp: 300,
        mp: 300,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 255,
            water: 255,
            wind: 255,
            earth: 255,
            light: 255,
            dark: 255,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK Test No.1",
        id: 10810102,
        monster_type: "MACHINE_SKILL",
        level: 30,
        scale: 0.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Fox Trot B3",
        id: 10810300,
        monster_type: "MACHINE",
        level: 62,
        scale: 0.7,
        exp: 6750,
        job: 7200,
        hp: 3700,
        sp: 300,
        mp: 300,
        def: [10, 200],
        mdef: [25, 65],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10017100, 0, /*null*/
            10023504, 10054200, 60051780, 10023506, 0,
        ],
    },
    Monster {
        name: "Fox Trot B3E",
        id: 10810301,
        monster_type: "MACHINE",
        level: 45,
        scale: 0.7,
        exp: 135,
        job: 144,
        hp: 3500,
        sp: 300,
        mp: 300,
        def: [10, 120],
        mdef: [10, 65],
        properties: Properties {
            fire: 70,
            water: 70,
            wind: 70,
            earth: 70,
            light: 70,
            dark: 70,
        },
        drop_ids: [
            10047412, 0, /*null*/
            0, /*null*/
            10047414, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Detroit T00",
        id: 10810302,
        monster_type: "MACHINE_SKILL",
        level: 1,
        scale: 0.5,
        exp: 6,
        job: 45,
        hp: 3,
        sp: 300,
        mp: 300,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Detroit T00",
        id: 10810303,
        monster_type: "MACHINE_SKILL",
        level: 1,
        scale: 0.5,
        exp: 6,
        job: 45,
        hp: 1,
        sp: 300,
        mp: 300,
        def: [99, 9999],
        mdef: [100, 9999],
        properties: Properties {
            fire: 255,
            water: 255,
            wind: 255,
            earth: 255,
            light: 255,
            dark: 255,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK Test No.2",
        id: 10810304,
        monster_type: "MACHINE_SKILL",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK Test No.3",
        id: 10810305,
        monster_type: "MACHINE_SKILL",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Fox Trot G2",
        id: 10810500,
        monster_type: "MACHINE",
        level: 55,
        scale: 0.7,
        exp: 2565,
        job: 2565,
        hp: 1600,
        sp: 300,
        mp: 300,
        def: [10, 180],
        mdef: [50, 53],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10017100, 0, /*null*/
            10023100, 10023502, 10025450, 10053900, 0,
        ],
    },
    Monster {
        name: "Fox Trot G2E",
        id: 10810501,
        monster_type: "MACHINE",
        level: 33,
        scale: 0.7,
        exp: 51,
        job: 51,
        hp: 2000,
        sp: 300,
        mp: 300,
        def: [10, 90],
        mdef: [10, 53],
        properties: Properties {
            fire: 70,
            water: 70,
            wind: 70,
            earth: 70,
            light: 70,
            dark: 70,
        },
        drop_ids: [
            10047412, 10047412, 0, /*null*/
            10047403, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fox Trot EE",
        id: 10810550,
        monster_type: "MACHINE",
        level: 50,
        scale: 2.5,
        exp: 384,
        job: 360,
        hp: 30000,
        sp: 300,
        mp: 300,
        def: [10, 180],
        mdef: [10, 53],
        properties: Properties {
            fire: 70,
            water: 70,
            wind: 70,
            earth: 70,
            light: 70,
            dark: 70,
        },
        drop_ids: [
            10047412, 10047412, 0, /*null*/
            10047403, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fox Trot C1",
        id: 10811200,
        monster_type: "MACHINE",
        level: 38,
        scale: 0.7,
        exp: 1575,
        job: 1890,
        hp: 1350,
        sp: 300,
        mp: 300,
        def: [10, 150],
        mdef: [35, 34],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10017100, 0, /*null*/
            10023504, 10023508, 10025450, 10023505, 0,
        ],
    },
    Monster {
        name: "Fox Trot C1E",
        id: 10811201,
        monster_type: "MACHINE",
        level: 23,
        scale: 0.7,
        exp: 33,
        job: 36,
        hp: 1300,
        sp: 300,
        mp: 300,
        def: [10, 60],
        mdef: [10, 80],
        properties: Properties {
            fire: 70,
            water: 70,
            wind: 70,
            earth: 70,
            light: 70,
            dark: 70,
        },
        drop_ids: [
            10047412, 0, /*null*/
            0, /*null*/
            10047413, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fox Trot C1EE",
        id: 10811202,
        monster_type: "MACHINE",
        level: 28,
        scale: 1.0,
        exp: 33,
        job: 36,
        hp: 1500,
        sp: 300,
        mp: 300,
        def: [10, 80],
        mdef: [10, 30],
        properties: Properties {
            fire: 70,
            water: 70,
            wind: 70,
            earth: 70,
            light: 70,
            dark: 70,
        },
        drop_ids: [
            10047412, 0, /*null*/
            0, /*null*/
            10047413, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Detroit T01",
        id: 10811203,
        monster_type: "MACHINE_SKILL_BOSS",
        level: 1,
        scale: 3.0,
        exp: 450,
        job: 540,
        hp: 37500,
        sp: 300,
        mp: 300,
        def: [30, 140],
        mdef: [5, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK Test No.4",
        id: 10811204,
        monster_type: "MACHINE_SKILL_BOSS",
        level: 30,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Fox Hound S5",
        id: 10820000,
        monster_type: "MACHINE",
        level: 83,
        scale: 1.1,
        exp: 14850,
        job: 14850,
        hp: 8500,
        sp: 300,
        mp: 300,
        def: [10, 125],
        mdef: [50, 65],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10016150, 10014854, 10019900, 10025451, 10025453, 10025453, 0,
        ],
    },
    Monster {
        name: "Robot Gatling",
        id: 10820001,
        monster_type: "MACHINE",
        level: 1,
        scale: 2.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fox Hound R1",
        id: 10820100,
        monster_type: "MACHINE",
        level: 38,
        scale: 1.0,
        exp: 1620,
        job: 1935,
        hp: 1400,
        sp: 300,
        mp: 300,
        def: [10, 180],
        mdef: [45, 40],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10017100, 10025900, 10026800, 10023100, 10023502, 10025500, 10053900, 0,
        ],
    },
    Monster {
        name: "Fox Hound R1E",
        id: 10820101,
        monster_type: "MACHINE",
        level: 23,
        scale: 0.7,
        exp: 33,
        job: 36,
        hp: 1000,
        sp: 300,
        mp: 300,
        def: [10, 80],
        mdef: [10, 20],
        properties: Properties {
            fire: 50,
            water: 50,
            wind: 50,
            earth: 50,
            light: 50,
            dark: 50,
        },
        drop_ids: [
            10047412, 0, /*null*/
            0, /*null*/
            10047403, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fox Hound R1EE",
        id: 10820102,
        monster_type: "MACHINE",
        level: 28,
        scale: 1.0,
        exp: 33,
        job: 36,
        hp: 1200,
        sp: 300,
        mp: 300,
        def: [10, 100],
        mdef: [10, 30],
        properties: Properties {
            fire: 50,
            water: 50,
            wind: 50,
            earth: 50,
            light: 50,
            dark: 50,
        },
        drop_ids: [
            10047412, 0, /*null*/
            0, /*null*/
            10047403, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fox Hound EEE",
        id: 10820150,
        monster_type: "MACHINE_BOSS_SKILL",
        level: 55,
        scale: 4.0,
        exp: 810,
        job: 765,
        hp: 150000,
        sp: 300,
        mp: 300,
        def: [10, 200],
        mdef: [40, 40],
        properties: Properties {
            fire: 80,
            water: 80,
            wind: 80,
            earth: 80,
            light: 80,
            dark: 80,
        },
        drop_ids: [
            10047412, 10047412, 0, /*null*/
            10047403, 10047403, 10047414, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fox Hound E200",
        id: 10820151,
        monster_type: "MACHINE_BOSS_SKILL",
        level: 5,
        scale: 4.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Fox Hound B3",
        id: 10820300,
        monster_type: "MACHINE",
        level: 64,
        scale: 1.0,
        exp: 7200,
        job: 7650,
        hp: 3800,
        sp: 300,
        mp: 300,
        def: [10, 210],
        mdef: [35, 70],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10017100, 10026004, 10026800, 10025700, 10023506, 10025550, 10023502, 0,
        ],
    },
    Monster {
        name: "Fox Hound B3E",
        id: 10820301,
        monster_type: "MACHINE",
        level: 42,
        scale: 1.0,
        exp: 144,
        job: 144,
        hp: 3500,
        sp: 300,
        mp: 300,
        def: [10, 120],
        mdef: [10, 70],
        properties: Properties {
            fire: 50,
            water: 50,
            wind: 50,
            earth: 50,
            light: 50,
            dark: 50,
        },
        drop_ids: [
            10047412, 0, /*null*/
            0, /*null*/
            10047414, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Genocide H00",
        id: 10820302,
        monster_type: "MACHINE_SKILL",
        level: 1,
        scale: 1.0,
        exp: 270,
        job: 360,
        hp: 1000,
        sp: 300,
        mp: 300,
        def: [20, 80],
        mdef: [50, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK Test No.5",
        id: 10820303,
        monster_type: "MACHINE_SKILL",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Fox Hound G4",
        id: 10820500,
        monster_type: "MACHINE",
        level: 70,
        scale: 1.0,
        exp: 11700,
        job: 10800,
        hp: 5500,
        sp: 300,
        mp: 300,
        def: [20, 300],
        mdef: [65, 70],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10017100, 10026050, 10026800, 10025700, 10023504, 10025551, 10023506, 0,
        ],
    },
    Monster {
        name: "Fox Hound C2",
        id: 10821200,
        monster_type: "MACHINE",
        level: 55,
        scale: 1.0,
        exp: 2520,
        job: 2520,
        hp: 1750,
        sp: 300,
        mp: 300,
        def: [10, 200],
        mdef: [50, 60],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10017100, 10026200, 10026800, 10023200, 10023503, 10025500, 10023508, 0,
        ],
    },
    Monster {
        name: "Fox Hound C2E",
        id: 10821201,
        monster_type: "MACHINE",
        level: 35,
        scale: 1.0,
        exp: 51,
        job: 51,
        hp: 1850,
        sp: 300,
        mp: 300,
        def: [10, 110],
        mdef: [10, 60],
        properties: Properties {
            fire: 50,
            water: 50,
            wind: 50,
            earth: 50,
            light: 50,
            dark: 50,
        },
        drop_ids: [
            10047412, 10047412, 0, /*null*/
            10047405, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gigant S6",
        id: 10830000,
        monster_type: "MACHINE",
        level: 85,
        scale: 1.3,
        exp: 15750,
        job: 16200,
        hp: 10000,
        sp: 300,
        mp: 300,
        def: [20, 300],
        mdef: [65, 75],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10030500, 10023200, 10025700, 10025551, 10025552, 10025552, 0,
        ],
    },
    Monster {
        name: "Robot Cannon",
        id: 10830001,
        monster_type: "MACHINE",
        level: 1,
        scale: 2.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gigant R3",
        id: 10830100,
        monster_type: "MACHINE",
        level: 65,
        scale: 1.3,
        exp: 7650,
        job: 7200,
        hp: 3850,
        sp: 300,
        mp: 300,
        def: [10, 230],
        mdef: [30, 60],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10017100, 0, /*null*/
            10026800, 10025600, 10023506, 10025400, 10023508, 0,
        ],
    },
    Monster {
        name: "Gigant R3E",
        id: 10830101,
        monster_type: "MACHINE",
        level: 44,
        scale: 1.3,
        exp: 153,
        job: 144,
        hp: 5000,
        sp: 300,
        mp: 300,
        def: [10, 180],
        mdef: [10, 60],
        properties: Properties {
            fire: 40,
            water: 40,
            wind: 40,
            earth: 40,
            light: 40,
            dark: 40,
        },
        drop_ids: [
            10047412, 0, /*null*/
            0, /*null*/
            10047414, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gigant B1",
        id: 10830300,
        monster_type: "MACHINE",
        level: 38,
        scale: 1.3,
        exp: 1620,
        job: 2025,
        hp: 1500,
        sp: 300,
        mp: 300,
        def: [10, 190],
        mdef: [35, 50],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10017100, 0, /*null*/
            10026800, 10023503, 10054300, 10025450, 10054100, 0,
        ],
    },
    Monster {
        name: "Gigant B1E",
        id: 10830301,
        monster_type: "MACHINE",
        level: 23,
        scale: 1.0,
        exp: 33,
        job: 42,
        hp: 1500,
        sp: 300,
        mp: 300,
        def: [10, 120],
        mdef: [10, 60],
        properties: Properties {
            fire: 40,
            water: 40,
            wind: 40,
            earth: 40,
            light: 40,
            dark: 40,
        },
        drop_ids: [
            10047412, 10047412, 10047412, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Storm G00",
        id: 10830302,
        monster_type: "MACHINE_SKILL_BOSS",
        level: 1,
        scale: 2.0,
        exp: 900,
        job: 2025,
        hp: 25200,
        sp: 300,
        mp: 300,
        def: [0, 10],
        mdef: [80, 160],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK Test No.6",
        id: 10830303,
        monster_type: "MACHINE_SKILL_BOSS",
        level: 30,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Gigant G2",
        id: 10830500,
        monster_type: "MACHINE",
        level: 58,
        scale: 1.3,
        exp: 2655,
        job: 2655,
        hp: 1950,
        sp: 300,
        mp: 300,
        def: [10, 210],
        mdef: [50, 70],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10017100, 0, /*null*/
            10026800, 10023505, 10023508, 10025450, 10023504, 0,
        ],
    },
    Monster {
        name: "Gigant G2E",
        id: 10830501,
        monster_type: "MACHINE",
        level: 36,
        scale: 1.3,
        exp: 51,
        job: 51,
        hp: 3000,
        sp: 300,
        mp: 300,
        def: [10, 150],
        mdef: [10, 70],
        properties: Properties {
            fire: 40,
            water: 40,
            wind: 40,
            earth: 40,
            light: 40,
            dark: 40,
        },
        drop_ids: [
            10047412, 10047412, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gigant C4",
        id: 10831200,
        monster_type: "MACHINE",
        level: 77,
        scale: 1.3,
        exp: 13275,
        job: 13050,
        hp: 6000,
        sp: 300,
        mp: 300,
        def: [30, 300],
        mdef: [70, 70],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10017100, 0, /*null*/
            10053300, 10053300, 10054200, 10025452, 10023503, 0,
        ],
    },
    Monster {
        name: "Gigant X7",
        id: 10830002,
        monster_type: "MACHINE",
        level: 40,
        scale: 2.0,
        exp: 3150,
        job: 3150,
        hp: 3500,
        sp: 300,
        mp: 300,
        def: [10, 40],
        mdef: [30, 20],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10023509, 10023509, 10023509, 10023509, 10023509, 10023509, 10023509, 0,
        ],
    },
    Monster {
        name: "Gigant XE",
        id: 10830003,
        monster_type: "MACHINE",
        level: 30,
        scale: 0.5,
        exp: 63,
        job: 63,
        hp: 2200,
        sp: 300,
        mp: 300,
        def: [10, 30],
        mdef: [10, 20],
        properties: Properties {
            fire: 40,
            water: 40,
            wind: 40,
            earth: 40,
            light: 40,
            dark: 40,
        },
        drop_ids: [
            10047412, 10047412, 0, /*null*/
            10047413, 10047404, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Bipedal Walking Robot",
        id: 10840000,
        monster_type: "MACHINE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Guttinger ?°",
        id: 10850000,
        monster_type: "MACHINE",
        level: 80,
        scale: 1.3,
        exp: 14850,
        job: 15750,
        hp: 16500,
        sp: 500,
        mp: 500,
        def: [75, 450],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10053300, 10018001, 10018002, 60051780, 10013300, 10023502, 10023504, 0,
        ],
    },
    Monster {
        name: "Guttinger ?c",
        id: 10850001,
        monster_type: "MACHINE",
        level: 87,
        scale: 1.1,
        exp: 18000,
        job: 19350,
        hp: 21500,
        sp: 500,
        mp: 500,
        def: [70, 440],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10009500, 10009500, 10025450, 60023180, 60023180, 10023503, 10023505, 0,
        ],
    },
    Monster {
        name: "Guttinger 7",
        id: 10850002,
        monster_type: "MACHINE",
        level: 95,
        scale: 1.3,
        exp: 22500,
        job: 22500,
        hp: 30000,
        sp: 500,
        mp: 500,
        def: [75, 455],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10009500, 10009500, 10025400, 10025452, 60040780, 60040780, 60040780, 0,
        ],
    },
    Monster {
        name: "Rebooted Guttinger",
        id: 10850003,
        monster_type: "MACHINE",
        level: 98,
        scale: 1.0,
        exp: 26100,
        job: 25650,
        hp: 38500,
        sp: 400,
        mp: 400,
        def: [70, 465],
        mdef: [55, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001550, 10033350, 10030500, 10054100, 10021851, 10030000, 10021000, 0,
        ],
    },
    Monster {
        name: "Guttinger ?EE",
        id: 10850004,
        monster_type: "MACHINE",
        level: 40,
        scale: 0.2,
        exp: 24,
        job: 24,
        hp: 3000,
        sp: 300,
        mp: 300,
        def: [0, 0],
        mdef: [10, 500],
        properties: Properties {
            fire: 255,
            water: 255,
            wind: 255,
            earth: 255,
            light: 255,
            dark: 255,
        },
        drop_ids: [
            0, /*null*/
            10047403, 10047405, 10047413, 10047404, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Purple Gatengar",
        id: 10850400,
        monster_type: "MACHINE",
        level: 40,
        scale: 0.2,
        exp: 24,
        job: 24,
        hp: 3000,
        sp: 300,
        mp: 300,
        def: [0, 0],
        mdef: [10, 500],
        properties: Properties {
            fire: 255,
            water: 255,
            wind: 255,
            earth: 255,
            light: 255,
            dark: 255,
        },
        drop_ids: [
            0, /*null*/
            10047403, 10047405, 10047413, 10047404, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Green Gatengar",
        id: 10850500,
        monster_type: "MACHINE",
        level: 40,
        scale: 0.2,
        exp: 24,
        job: 24,
        hp: 3000,
        sp: 300,
        mp: 300,
        def: [0, 0],
        mdef: [10, 500],
        properties: Properties {
            fire: 255,
            water: 255,
            wind: 255,
            earth: 255,
            light: 255,
            dark: 255,
        },
        drop_ids: [
            0, /*null*/
            10047403, 10047405, 10047413, 10047404, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Umbrella",
        id: 10860000,
        monster_type: "MAGIC_CREATURE",
        level: 79,
        scale: 0.7,
        exp: 13500,
        job: 14625,
        hp: 12350,
        sp: 400,
        mp: 400,
        def: [60, 235],
        mdef: [55, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10009101, 10001200, 10054900, 10054900, 60061400, 0,
        ],
    },
    Monster {
        name: "Red Umbrella",
        id: 10860100,
        monster_type: "MAGIC_CREATURE",
        level: 79,
        scale: 0.7,
        exp: 13500,
        job: 14625,
        hp: 12350,
        sp: 400,
        mp: 400,
        def: [60, 235],
        mdef: [55, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10009101, 10001200, 10054900, 10054900, 60061400, 0,
        ],
    },
    Monster {
        name: "Blue Umbrella",
        id: 10860400,
        monster_type: "MAGIC_CREATURE",
        level: 79,
        scale: 0.7,
        exp: 13500,
        job: 14625,
        hp: 12350,
        sp: 400,
        mp: 400,
        def: [60, 235],
        mdef: [55, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10009101, 10001200, 10054900, 10054900, 60061400, 0,
        ],
    },
    Monster {
        name: "Green Umbrella",
        id: 10860500,
        monster_type: "MAGIC_CREATURE",
        level: 79,
        scale: 0.7,
        exp: 13500,
        job: 14625,
        hp: 12350,
        sp: 400,
        mp: 400,
        def: [60, 235],
        mdef: [55, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10009101, 10001200, 10054900, 10054900, 60061400, 0,
        ],
    },
    Monster {
        name: "Yellow Umbrella",
        id: 10860700,
        monster_type: "MAGIC_CREATURE",
        level: 79,
        scale: 0.7,
        exp: 13500,
        job: 14625,
        hp: 12350,
        sp: 400,
        mp: 400,
        def: [60, 235],
        mdef: [55, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10009101, 10001200, 10054900, 10054900, 60061400, 0,
        ],
    },
    Monster {
        name: "Black Umbrella",
        id: 10861000,
        monster_type: "MAGIC_CREATURE",
        level: 79,
        scale: 0.7,
        exp: 13500,
        job: 14625,
        hp: 12350,
        sp: 400,
        mp: 400,
        def: [60, 235],
        mdef: [55, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10009101, 10001200, 10054900, 10054900, 60061400, 0,
        ],
    },
    Monster {
        name: "White Umbrella",
        id: 10861400,
        monster_type: "MAGIC_CREATURE",
        level: 79,
        scale: 0.7,
        exp: 13500,
        job: 14625,
        hp: 12350,
        sp: 400,
        mp: 400,
        def: [60, 235],
        mdef: [55, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10009101, 10001200, 10054900, 10054900, 60061400, 0,
        ],
    },
    Monster {
        name: "Lilicurl",
        id: 10860010,
        monster_type: "MAGIC_CREATURE",
        level: 90,
        scale: 0.7,
        exp: 14400,
        job: 15075,
        hp: 12550,
        sp: 400,
        mp: 400,
        def: [50, 215],
        mdef: [50, 65],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037505, 10005646, 10019900, 10025051, 60030800, 61030054, 0,
        ],
    },
    Monster {
        name: "Wendy",
        id: 10860017,
        monster_type: "MAGIC_CREATURE",
        level: 91,
        scale: 0.7,
        exp: 15300,
        job: 16425,
        hp: 13350,
        sp: 400,
        mp: 400,
        def: [60, 235],
        mdef: [55, 75],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037505, 10005646, 10019900, 60061400, 60030253, 60060450, 0,
        ],
    },
    Monster {
        name: "Crystal Urchin",
        id: 10870000,
        monster_type: "ROCK",
        level: 55,
        scale: 1.6,
        exp: 2520,
        job: 2745,
        hp: 2000,
        sp: 300,
        mp: 300,
        def: [15, 100],
        mdef: [30, 55],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014854, 10025250, 10014300, 0, /*null*/
            10015707, 10025210, 0,
        ],
    },
    Monster {
        name: "Fanis",
        id: 10870100,
        monster_type: "ROCK",
        level: 88,
        scale: 1.6,
        exp: 11250,
        job: 10800,
        hp: 6800,
        sp: 113,
        mp: 299,
        def: [10, 115],
        mdef: [30, 110],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10016150, 10015708, 10019900, 10054900, 10013300, 10013300, 0,
        ],
    },
    Monster {
        name: "Maleable",
        id: 10870300,
        monster_type: "ROCK",
        level: 36,
        scale: 1.3,
        exp: 1485,
        job: 1689,
        hp: 1200,
        sp: 105,
        mp: 250,
        def: [15, 111],
        mdef: [20, 120],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10033904, 10011203, 10011000, 0, /*null*/
            10046300, 10013300, 0,
        ],
    },
    Monster {
        name: "Diterites",
        id: 10870500,
        monster_type: "ROCK",
        level: 92,
        scale: 1.6,
        exp: 15750,
        job: 15300,
        hp: 12000,
        sp: 250,
        mp: 250,
        def: [5, 120],
        mdef: [25, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037505, 10033350, 10000302, 10019900, 10046400, 10025206, 0,
        ],
    },
    Monster {
        name: "Twinkle",
        id: 10870700,
        monster_type: "ROCK",
        level: 66,
        scale: 1.6,
        exp: 6525,
        job: 5400,
        hp: 2250,
        sp: 300,
        mp: 300,
        def: [10, 130],
        mdef: [50, 145],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014854, 10025250, 10003750, 10014852, 10014852, 10054800, 0,
        ],
    },
    Monster {
        name: "Black Crystal Urchin",
        id: 10870900,
        monster_type: "ROCK",
        level: 73,
        scale: 1.0,
        exp: 441,
        job: 504,
        hp: 100,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003750, 10015250, 10026350, 0, /*null*/
            10014854, 10025250, 0,
        ],
    },
    Monster {
        name: "Sellslay",
        id: 10871100,
        monster_type: "ROCK",
        level: 83,
        scale: 1.6,
        exp: 10575,
        job: 11250,
        hp: 6300,
        sp: 113,
        mp: 299,
        def: [35, 140],
        mdef: [60, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10014650, 10038400, 10019900, 10054800, 10013300, 10013300, 0,
        ],
    },
    Monster {
        name: "Grasha",
        id: 10871400,
        monster_type: "ROCK",
        level: 76,
        scale: 1.6,
        exp: 11925,
        job: 9900,
        hp: 4200,
        sp: 300,
        mp: 300,
        def: [10, 165],
        mdef: [40, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025250, 10014854, 10014851, 10013300, 10013300, 10054800, 0,
        ],
    },
    Monster {
        name: "Wild Dragon",
        id: 10960000,
        monster_type: "ANIMAL",
        level: 95,
        scale: 1.5,
        exp: 26100,
        job: 26550,
        hp: 40500,
        sp: 400,
        mp: 400,
        def: [36, 264],
        mdef: [44, 56],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10006402, 10006402, 10006652, 10006402, 10009500, 10015304, 10037700, 0,
        ],
    },
    Monster {
        name: "Mother Dragon",
        id: 10960001,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 110,
        scale: 4.0,
        exp: 306000,
        job: 328500,
        hp: 750000,
        sp: 500,
        mp: 500,
        def: [70, 500],
        mdef: [60, 450],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10015304, 10015304, 10009500, 10015304, 10009500, 10050850, 10049500, 0,
        ],
    },
    Monster {
        name: "Holograph",
        id: 10970000,
        monster_type: "MACHINE",
        level: 93,
        scale: 1.0,
        exp: 20925,
        job: 20025,
        hp: 15500,
        sp: 400,
        mp: 400,
        def: [40, 230],
        mdef: [50, 60],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10030500, 10015708, 10038400, 10021000, 10017000, 10025553, 0,
        ],
    },
    Monster {
        name: "Satellite Eye E",
        id: 10970001,
        monster_type: "MACHINE_SKILL",
        level: 10,
        scale: 1.0,
        exp: 18,
        job: 18,
        hp: 240,
        sp: 400,
        mp: 400,
        def: [0, 10],
        mdef: [0, 10],
        properties: Properties {
            fire: 255,
            water: 255,
            wind: 255,
            earth: 255,
            light: 255,
            dark: 255,
        },
        drop_ids: [
            10047414, 10047407, 10047403, 10047407, 10047403, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Holograph CD",
        id: 10970002,
        monster_type: "MACHINE",
        level: 93,
        scale: 0.7,
        exp: 20925,
        job: 20025,
        hp: 15500,
        sp: 400,
        mp: 400,
        def: [40, 230],
        mdef: [50, 60],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10030500, 10015708, 10038400, 10021000, 10017000, 10025553, 0,
        ],
    },
    Monster {
        name: "Error Searchbot",
        id: 10970900,
        monster_type: "MACHINE",
        level: 27,
        scale: 1.0,
        exp: 810,
        job: 1215,
        hp: 700,
        sp: 400,
        mp: 400,
        def: [15, 48],
        mdef: [10, 28],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020209, 10019900, 0, /*null*/
            90000043, 10020209, 10023504, 0,
        ],
    },
    Monster {
        name: "Error Searchbot",
        id: 10970901,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 1,
        scale: 1.3,
        exp: 810,
        job: 12150,
        hp: 34500,
        sp: 400,
        mp: 400,
        def: [10, 25],
        mdef: [12, 28],
        properties: Properties {
            fire: 23,
            water: 26,
            wind: 52,
            earth: 28,
            light: 22,
            dark: 45,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Massive Seeker",
        id: 10970902,
        monster_type: "MACHINE_SKILL",
        level: 1,
        scale: 1.0,
        exp: 810,
        job: 1215,
        hp: 800,
        sp: 400,
        mp: 400,
        def: [15, 48],
        mdef: [10, 28],
        properties: Properties {
            fire: 10,
            water: 12,
            wind: 25,
            earth: 25,
            light: 10,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK Test No.7",
        id: 10970903,
        monster_type: "MACHINE_SKILL",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Search Eye",
        id: 10971700,
        monster_type: "MACHINE",
        level: 20,
        scale: 1.0,
        exp: 441,
        job: 663,
        hp: 300,
        sp: 100,
        mp: 100,
        def: [15, 30],
        mdef: [10, 20],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020209, 10019900, 0, /*null*/
            90000045, 10020209, 10023505, 0,
        ],
    },
    Monster {
        name: "Bewhilder",
        id: 10971701,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 1,
        scale: 1.2,
        exp: 4410,
        job: 6615,
        hp: 29500,
        sp: 100,
        mp: 100,
        def: [8, 20],
        mdef: [10, 20],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Search Eye",
        id: 10971702,
        monster_type: "MACHINE_SKILL",
        level: 1,
        scale: 1.0,
        exp: 441,
        job: 663,
        hp: 600,
        sp: 100,
        mp: 100,
        def: [15, 30],
        mdef: [10, 20],
        properties: Properties {
            fire: 22,
            water: 32,
            wind: 42,
            earth: 42,
            light: 22,
            dark: 42,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Search Eye?A",
        id: 10971703,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 30,
        scale: 1.0,
        exp: 13500,
        job: 19260,
        hp: 40000,
        sp: 100,
        mp: 100,
        def: [13, 30],
        mdef: [8, 22],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10067302, 0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Killer Seeker",
        id: 10971704,
        monster_type: "MACHINE",
        level: 44,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK Test No.9",
        id: 10971705,
        monster_type: "MACHINE_SKILL",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Escargot",
        id: 10980000,
        monster_type: "PLANT",
        level: 75,
        scale: 1.0,
        exp: 11025,
        job: 11250,
        hp: 4750,
        sp: 400,
        mp: 400,
        def: [20, 220],
        mdef: [50, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10015302, 10051600, 10034507, 10001805, 10034507, 10034507, 10021500, 0,
        ],
    },
    Monster {
        name: "Balulu Child",
        id: 10990000,
        monster_type: "ANIMAL",
        level: 94,
        scale: 0.8,
        exp: 19800,
        job: 20250,
        hp: 15400,
        sp: 400,
        mp: 400,
        def: [60, 235],
        mdef: [55, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10006600, 10020206, 10006400, 10019900, 10054600, 10020352, 61020452, 0,
        ],
    },
    Monster {
        name: "Horn Beast",
        id: 10990001,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 100,
        scale: 2.0,
        exp: 202500,
        job: 202500,
        hp: 525000,
        sp: 500,
        mp: 500,
        def: [65, 500],
        mdef: [50, 300],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10006400, 10037300, 10054600, 10020353, 61020453, 10020352, 61020453, 0,
        ],
    },
    Monster {
        name: "Balulu",
        id: 10990002,
        monster_type: "ANIMAL",
        level: 94,
        scale: 1.0,
        exp: 21600,
        job: 22500,
        hp: 15000,
        sp: 400,
        mp: 400,
        def: [42, 235],
        mdef: [40, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 40,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10006600, 10020206, 10006400, 10019900, 10054600, 10020352, 61020452, 0,
        ],
    },
    Monster {
        name: "Flying Bat Umbrella",
        id: 14020000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20000,
        sp: 1,
        mp: 1,
        def: [80, 100],
        mdef: [50, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gagit",
        id: 14040000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20000,
        sp: 1,
        mp: 1,
        def: [80, 100],
        mdef: [50, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Titania Dragon",
        id: 14110000,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 118,
        scale: 0.8,
        exp: 360000,
        job: 342000,
        hp: 958000,
        sp: 0,
        mp: 0,
        def: [60, 400],
        mdef: [60, 500],
        properties: Properties {
            fire: 30,
            water: 10,
            wind: 10,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10006402, 10020404, 10009600, 10050300, 0, /*null*/
            10037700, 10049500, 0,
        ],
    },
    Monster {
        name: "Inferior Dragon",
        id: 14110001,
        monster_type: "ANIMAL",
        level: 90,
        scale: 0.15,
        exp: 18000,
        job: 18900,
        hp: 16000,
        sp: 0,
        mp: 0,
        def: [50, 250],
        mdef: [60, 150],
        properties: Properties {
            fire: 5,
            water: 5,
            wind: 5,
            earth: 5,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10024700, 10024700, 10024700, 10024700, 10024700, 10024700, 10024700, 0,
        ],
    },
    Monster {
        name: "The Ruler of Titania",
        id: 14110002,
        monster_type: "ANIMAL_SPBOSS_SKILL_NOTPTDROPRANGE",
        level: 200,
        scale: 0.9,
        exp: 1057500,
        job: 477000,
        hp: 1550000,
        sp: 0,
        mp: 0,
        def: [65, 550],
        mdef: [75, 650],
        properties: Properties {
            fire: 10,
            water: 30,
            wind: 30,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10031151, 10031151, 10031151, 10031152, 10031152, 10031152, 10031152, 0,
        ],
    },
    Monster {
        name: "Sleiph Dragon",
        id: 14110003,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 98,
        scale: 0.3,
        exp: 22500,
        job: 23400,
        hp: 38000,
        sp: 0,
        mp: 0,
        def: [50, 300],
        mdef: [60, 200],
        properties: Properties {
            fire: 5,
            water: 5,
            wind: 5,
            earth: 5,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006402, 0, /*null*/
            10031150, 10031150, 10031150, 10031150, 0,
        ],
    },
    Monster {
        name: "Sight Dragon",
        id: 14110004,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 98,
        scale: 0.5,
        exp: 22500,
        job: 23400,
        hp: 160000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 100,
            dark: 0,
        },
        drop_ids: [
            10002807, 10006402, 10019900, 10067302, 10009101, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Crystal Turtle",
        id: 14120000,
        monster_type: "ROCK",
        level: 92,
        scale: 1.0,
        exp: 29070,
        job: 30600,
        hp: 56000,
        sp: 0,
        mp: 0,
        def: [70, 300],
        mdef: [50, 250],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10034502, 10062300, 0, /*null*/
            10005200, 50073710, 0,
        ],
    },
    Monster {
        name: "Stone Turtle",
        id: 14120500,
        monster_type: "ROCK",
        level: 40,
        scale: 1.0,
        exp: 2475,
        job: 2925,
        hp: 5000,
        sp: 0,
        mp: 0,
        def: [60, 200],
        mdef: [30, 50],
        properties: Properties {
            fire: 0,
            water: 10,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10034502, 10062300, 0, /*null*/
            10005200, 50073710, 0,
        ],
    },
    Monster {
        name: "Stone Turtle Sage",
        id: 14120501,
        monster_type: "ROCK_BOSS_SKILL_NOTPTDROPRANGE",
        level: 100,
        scale: 3.0,
        exp: 225000,
        job: 270000,
        hp: 750000,
        sp: 0,
        mp: 0,
        def: [60, 700],
        mdef: [60, 300],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 20,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10034502, 10012100, 10062300, 50073709, 10049500, 50073709, 10049500, 0,
        ],
    },
    Monster {
        name: "Sea Horse",
        id: 14130000,
        monster_type: "WATER_ANIMAL",
        level: 83,
        scale: 1.0,
        exp: 12600,
        job: 14850,
        hp: 15000,
        sp: 0,
        mp: 0,
        def: [35, 150],
        mdef: [35, 150],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10062000, 10001800, 10061600, 0, /*null*/
            10033350, 10061602, 0,
        ],
    },
    Monster {
        name: "Sea Dragon",
        id: 14130400,
        monster_type: "WATER_ANIMAL",
        level: 73,
        scale: 1.0,
        exp: 11250,
        job: 12600,
        hp: 13580,
        sp: 0,
        mp: 0,
        def: [30, 145],
        mdef: [30, 140],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10037505, 10062000, 0, /*null*/
            10018204, 10061602, 0,
        ],
    },
    Monster {
        name: "Elemental",
        id: 14140000,
        monster_type: "MAGIC_CREATURE",
        level: 84,
        scale: 1.0,
        exp: 16155,
        job: 18675,
        hp: 69160,
        sp: 0,
        mp: 0,
        def: [50, 125],
        mdef: [12, 80],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001800, 10033350, 10034504, 10054900, 0, /*null*/
            10014500, 10014300, 0,
        ],
    },
    Monster {
        name: "Hermit Slug",
        id: 14150000,
        monster_type: "INSECT",
        level: 30,
        scale: 1.0,
        exp: 1125,
        job: 1350,
        hp: 1500,
        sp: 0,
        mp: 0,
        def: [15, 55],
        mdef: [35, 40],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009150, 10033910, 10061800, 10043400, 10061700, 10061703, 0,
        ],
    },
    Monster {
        name: "Hermit Crab",
        id: 14150200,
        monster_type: "INSECT",
        level: 72,
        scale: 1.0,
        exp: 10800,
        job: 11475,
        hp: 11000,
        sp: 0,
        mp: 0,
        def: [30, 135],
        mdef: [20, 135],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10000409, 10061800, 10043400, 10061900, 10061702, 0,
        ],
    },
    Monster {
        name: "Conch",
        id: 14150800,
        monster_type: "INSECT",
        level: 82,
        scale: 1.0,
        exp: 12375,
        job: 13950,
        hp: 13580,
        sp: 0,
        mp: 0,
        def: [35, 135],
        mdef: [20, 140],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003200, 10045550, 10061800, 10043400, 10061900, 10061750, 0,
        ],
    },
    Monster {
        name: "Balloon Jell",
        id: 14160000,
        monster_type: "WATER_ANIMAL",
        level: 87,
        scale: 1.0,
        exp: 17100,
        job: 18000,
        hp: 23580,
        sp: 0,
        mp: 0,
        def: [45, 220],
        mdef: [55, 80],
        properties: Properties {
            fire: 0,
            water: 45,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10034506, 90000045, 0, /*null*/
            10032801, 10021500, 0,
        ],
    },
    Monster {
        name: "Ballon Jell Chief",
        id: 14160001,
        monster_type: "WATER_ANIMAL_BOSS_SKILL",
        level: 97,
        scale: 2.0,
        exp: 26100,
        job: 28350,
        hp: 43580,
        sp: 0,
        mp: 0,
        def: [55, 350],
        mdef: [55, 220],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001800, 10001800, 10034506, 90000045, 10067302, 10032801, 10021500, 0,
        ],
    },
    Monster {
        name: "Poison Jell",
        id: 14160500,
        monster_type: "WATER_ANIMAL",
        level: 77,
        scale: 1.0,
        exp: 12600,
        job: 14400,
        hp: 19800,
        sp: 0,
        mp: 0,
        def: [30, 180],
        mdef: [35, 50],
        properties: Properties {
            fire: 0,
            water: 45,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003200, 10000507, 90000045, 0, /*null*/
            10032805, 10021500, 0,
        ],
    },
    Monster {
        name: "Poison Jell Chief",
        id: 14160501,
        monster_type: "WATER_ANIMAL_BOSS_SKILL",
        level: 87,
        scale: 2.0,
        exp: 24750,
        job: 26100,
        hp: 39800,
        sp: 0,
        mp: 0,
        def: [45, 300],
        mdef: [45, 200],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001800, 10001800, 10000507, 90000045, 10067302, 10032805, 10021500, 0,
        ],
    },
    Monster {
        name: "Sea Serpent",
        id: 14170000,
        monster_type: "WATER_ANIMAL",
        level: 88,
        scale: 1.0,
        exp: 24525,
        job: 24975,
        hp: 84700,
        sp: 0,
        mp: 0,
        def: [24, 105],
        mdef: [14, 60],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006402, 10006402, 10011000, 10020404, 10012551, 10037700, 0,
        ],
    },
    Monster {
        name: "Sea Serpent(Red)",
        id: 14170100,
        monster_type: "WATER_ANIMAL",
        level: 88,
        scale: 1.0,
        exp: 20025,
        job: 20475,
        hp: 32350,
        sp: 0,
        mp: 0,
        def: [65, 230],
        mdef: [45, 140],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006402, 10006402, 10011000, 10020404, 10012551, 10037700, 0,
        ],
    },
    Monster {
        name: "Sea Serpent(Ice)",
        id: 14170400,
        monster_type: "WATER_ANIMAL",
        level: 88,
        scale: 1.0,
        exp: 20025,
        job: 20475,
        hp: 32350,
        sp: 0,
        mp: 0,
        def: [65, 230],
        mdef: [45, 140],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006402, 10006402, 10011000, 10020404, 10012551, 10037700, 0,
        ],
    },
    Monster {
        name: "Sea Serpent(Green)",
        id: 14170500,
        monster_type: "WATER_ANIMAL",
        level: 88,
        scale: 1.0,
        exp: 20025,
        job: 20475,
        hp: 32350,
        sp: 0,
        mp: 0,
        def: [65, 230],
        mdef: [45, 140],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006402, 10006402, 10011000, 10020404, 10012551, 10037700, 0,
        ],
    },
    Monster {
        name: "Sea Serpent(Yellow)",
        id: 14170700,
        monster_type: "WATER_ANIMAL",
        level: 88,
        scale: 1.0,
        exp: 20025,
        job: 20475,
        hp: 32350,
        sp: 0,
        mp: 0,
        def: [65, 230],
        mdef: [45, 140],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006402, 10006402, 10011000, 10020404, 10012551, 10037700, 0,
        ],
    },
    Monster {
        name: "Ikuchi",
        id: 14170900,
        monster_type: "WATER_ANIMAL",
        level: 88,
        scale: 1.0,
        exp: 20025,
        job: 20475,
        hp: 32350,
        sp: 0,
        mp: 0,
        def: [65, 230],
        mdef: [45, 140],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006402, 10006402, 10011000, 10020404, 10012551, 10037700, 0,
        ],
    },
    Monster {
        name: "Sea Serpent(White)",
        id: 14171400,
        monster_type: "WATER_ANIMAL",
        level: 88,
        scale: 1.0,
        exp: 20025,
        job: 20475,
        hp: 32350,
        sp: 0,
        mp: 0,
        def: [65, 230],
        mdef: [45, 140],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006402, 10006402, 10011000, 10020404, 10012551, 10037700, 0,
        ],
    },
    Monster {
        name: "Dominion Dragon",
        id: 14260000,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 150,
        scale: 0.8,
        exp: 405000,
        job: 398250,
        hp: 1000000,
        sp: 0,
        mp: 0,
        def: [60, 430],
        mdef: [55, 550],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dominion Dragon",
        id: 14260001,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 30,
        scale: 0.8,
        exp: 0,
        job: 0,
        hp: 3500,
        sp: 0,
        mp: 0,
        def: [10, 40],
        mdef: [30, 20],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            10011700, 10011700, 10011700, 10011700, 10011700, 10011700, 10011700, 0,
        ],
    },
    Monster {
        name: "Overseeing Woman",
        id: 14260002,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Skink",
        id: 14270000,
        monster_type: "ANIMAL",
        level: 18,
        scale: 1.0,
        exp: 375,
        job: 612,
        hp: 620,
        sp: 0,
        mp: 0,
        def: [25, 25],
        mdef: [20, 45],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020304, 10015250, 0, /*null*/
            10049000, 10015250, 61020600, 0,
        ],
    },
    Monster {
        name: "Love Skink",
        id: 14270001,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 0,
        mp: 0,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10048566, 10066100, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Skink",
        id: 14270002,
        monster_type: "ANIMAL_SKILL",
        level: 1,
        scale: 1.0,
        exp: 375,
        job: 612,
        hp: 670,
        sp: 0,
        mp: 0,
        def: [8, 25],
        mdef: [12, 45],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Magi",
        id: 14270100,
        monster_type: "ANIMAL",
        level: 26,
        scale: 1.0,
        exp: 879,
        job: 2835,
        hp: 795,
        sp: 0,
        mp: 0,
        def: [10, 75],
        mdef: [10, 75],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020304, 10015250, 0, /*null*/
            10049000, 10015250, 61020600, 0,
        ],
    },
    Monster {
        name: "Magi",
        id: 14270101,
        monster_type: "ANIMAL",
        level: 1,
        scale: 1.0,
        exp: 351,
        job: 546,
        hp: 845,
        sp: 0,
        mp: 0,
        def: [12, 55],
        mdef: [8, 60],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Muffler Lizard",
        id: 14270900,
        monster_type: "ANIMAL_SKILL",
        level: 1,
        scale: 1.0,
        exp: 270,
        job: 1530,
        hp: 3000,
        sp: 0,
        mp: 0,
        def: [20, 60],
        mdef: [88, 60],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Muffler Lizard (Rainbow)",
        id: 14271500,
        monster_type: "ANIMAL",
        level: 36,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1900,
        sp: 0,
        mp: 0,
        def: [60, 400],
        mdef: [60, 500],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fiery Salamandra",
        id: 14280000,
        monster_type: "MAGIC_CREATURE",
        level: 27,
        scale: 1.0,
        exp: 915,
        job: 2364,
        hp: 1150,
        sp: 0,
        mp: 0,
        def: [15, 45],
        mdef: [15, 50],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10011201, 10019900, 0, /*null*/
            10012552, 10011201, 10011000, 0,
        ],
    },
    Monster {
        name: "Bliss Salamandra",
        id: 14280400,
        monster_type: "MAGIC_CREATURE",
        level: 27,
        scale: 1.0,
        exp: 915,
        job: 2364,
        hp: 1150,
        sp: 0,
        mp: 0,
        def: [15, 45],
        mdef: [15, 50],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10011201, 10019900, 0, /*null*/
            10012552, 10011201, 10011000, 0,
        ],
    },
    Monster {
        name: "Calm Salamandra",
        id: 14280600,
        monster_type: "MAGIC_CREATURE",
        level: 27,
        scale: 1.0,
        exp: 915,
        job: 2364,
        hp: 1150,
        sp: 0,
        mp: 0,
        def: [15, 45],
        mdef: [15, 50],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10011201, 10019900, 0, /*null*/
            10012552, 10011201, 10011000, 0,
        ],
    },
    Monster {
        name: "Malus",
        id: 14290000,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 36,
        job: 33,
        hp: 80,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10002800, 10019900, 0, /*null*/
            90000043, 10005612, 10005611, 0,
        ],
    },
    Monster {
        name: "Marius",
        id: 14290001,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 36,
        job: 33,
        hp: 80,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Poison Malus",
        id: 14291100,
        monster_type: "PLANT",
        level: 12,
        scale: 1.0,
        exp: 171,
        job: 285,
        hp: 300,
        sp: 0,
        mp: 0,
        def: [10, 20],
        mdef: [15, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001903, 10003200, 0, /*null*/
            90000043, 10002806, 10005611, 0,
        ],
    },
    Monster {
        name: "Poison Marius",
        id: 14291101,
        monster_type: "PLANT",
        level: 1,
        scale: 1.0,
        exp: 171,
        job: 285,
        hp: 320,
        sp: 0,
        mp: 0,
        def: [5, 20],
        mdef: [15, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gochin",
        id: 14300000,
        monster_type: "ROCK",
        level: 8,
        scale: 1.0,
        exp: 123,
        job: 303,
        hp: 130,
        sp: 0,
        mp: 0,
        def: [10, 5],
        mdef: [12, 15],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10038001, 10019900, 0, /*null*/
            90000044, 10015700, 90000044, 0,
        ],
    },
    Monster {
        name: "Gochin",
        id: 14300001,
        monster_type: "ROCK",
        level: 1,
        scale: 1.0,
        exp: 123,
        job: 303,
        hp: 130,
        sp: 0,
        mp: 0,
        def: [10, 5],
        mdef: [12, 15],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Gochin(Test)",
        id: 14300002,
        monster_type: "ROCK",
        level: 1,
        scale: 1.0,
        exp: 123,
        job: 303,
        hp: 130,
        sp: 0,
        mp: 0,
        def: [10, 5],
        mdef: [12, 15],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Flame Gochin",
        id: 14300100,
        monster_type: "ROCK",
        level: 22,
        scale: 1.0,
        exp: 474,
        job: 762,
        hp: 580,
        sp: 0,
        mp: 0,
        def: [28, 40],
        mdef: [25, 30],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001550, 10019900, 0, /*null*/
            90000044, 10022309, 90000044, 0,
        ],
    },
    Monster {
        name: "Flame Gochin",
        id: 14300101,
        monster_type: "ROCK",
        level: 1,
        scale: 1.0,
        exp: 474,
        job: 762,
        hp: 600,
        sp: 0,
        mp: 0,
        def: [12, 40],
        mdef: [25, 30],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Harmful Gochin",
        id: 14300200,
        monster_type: "ROCK",
        level: 24,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 670,
        sp: 0,
        mp: 0,
        def: [60, 400],
        mdef: [60, 500],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Harmful Gochin",
        id: 14300201,
        monster_type: "ROCK_SKILL",
        level: 1,
        scale: 1.1,
        exp: 819,
        job: 2457,
        hp: 1850,
        sp: 0,
        mp: 0,
        def: [14, 60],
        mdef: [20, 42],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Banpi Rock",
        id: 14300500,
        monster_type: "ROCK_SKILL",
        level: 1,
        scale: 1.1,
        exp: 1959,
        job: 5670,
        hp: 3600,
        sp: 0,
        mp: 0,
        def: [15, 65],
        mdef: [21, 43],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Banpi Rock",
        id: 14300501,
        monster_type: "ROCK_NOTPTDROPRANGE",
        level: 20,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 400,
        sp: 0,
        mp: 0,
        def: [10, 28],
        mdef: [21, 43],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031161, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Brazier Gochin",
        id: 14300900,
        monster_type: "ROCK",
        level: 30,
        scale: 1.0,
        exp: 1689,
        job: 5469,
        hp: 1358,
        sp: 0,
        mp: 0,
        def: [20, 70],
        mdef: [20, 55],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10038001, 10019900, 0, /*null*/
            90000044, 10015700, 90000044, 0,
        ],
    },
    Monster {
        name: "Frazzle Gochin",
        id: 14300901,
        monster_type: "ROCK_SKILL",
        level: 1,
        scale: 1.0,
        exp: 1689,
        job: 5469,
        hp: 1358,
        sp: 0,
        mp: 0,
        def: [14, 70],
        mdef: [20, 40],
        properties: Properties {
            fire: 10,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Obscure Rock",
        id: 14301000,
        monster_type: "ROCK_SKILL",
        level: 1,
        scale: 1.2,
        exp: 1800,
        job: 5625,
        hp: 5250,
        sp: 0,
        mp: 0,
        def: [24, 80],
        mdef: [21, 45],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "MultiColored Rock",
        id: 14301700,
        monster_type: "ROCK_BOMB_SKILL",
        level: 1,
        scale: 0.6,
        exp: 2724,
        job: 4047,
        hp: 15080,
        sp: 0,
        mp: 0,
        def: [30, 100],
        mdef: [22, 50],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Maoa",
        id: 14310000,
        monster_type: "MACHINE",
        level: 16,
        scale: 1.0,
        exp: 204,
        job: 339,
        hp: 450,
        sp: 0,
        mp: 0,
        def: [20, 30],
        mdef: [13, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015500, 10019900, 0, /*null*/
            90000044, 10000701, 10023502, 0,
        ],
    },
    Monster {
        name: "Maoa",
        id: 14310001,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 2025,
        job: 3375,
        hp: 29500,
        sp: 0,
        mp: 0,
        def: [12, 2],
        mdef: [35, 30],
        properties: Properties {
            fire: 9,
            water: 9,
            wind: 9,
            earth: 9,
            light: 2,
            dark: 9,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Maoa",
        id: 14310002,
        monster_type: "MACHINE",
        level: 1,
        scale: 1.0,
        exp: 204,
        job: 339,
        hp: 300,
        sp: 0,
        mp: 0,
        def: [5, 0],
        mdef: [33, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Maoa ?A",
        id: 14310003,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 30,
        scale: 1.0,
        exp: 11250,
        job: 17100,
        hp: 45000,
        sp: 0,
        mp: 0,
        def: [10, 40],
        mdef: [3, 22],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10067302, 0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blood Sucker",
        id: 14310100,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 2250,
        job: 3600,
        hp: 24500,
        sp: 0,
        mp: 0,
        def: [8, 12],
        mdef: [38, 20],
        properties: Properties {
            fire: 6,
            water: 6,
            wind: 6,
            earth: 6,
            light: 1,
            dark: 6,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blood Sucker",
        id: 14310101,
        monster_type: "MACHINE",
        level: 1,
        scale: 1.0,
        exp: 225,
        job: 360,
        hp: 500,
        sp: 0,
        mp: 0,
        def: [8, 10],
        mdef: [36, 20],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Auto Medic",
        id: 14310102,
        monster_type: "MACHINE",
        level: 45,
        scale: 0.8,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Auto Medic",
        id: 14310103,
        monster_type: "MACHINE",
        level: 45,
        scale: 0.8,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK Test No.9",
        id: 14310104,
        monster_type: "MACHINE",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Revealer",
        id: 14310200,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 2700,
        job: 3870,
        hp: 40000,
        sp: 0,
        mp: 0,
        def: [16, 17],
        mdef: [40, 30],
        properties: Properties {
            fire: 12,
            water: 12,
            wind: 12,
            earth: 12,
            light: 3,
            dark: 12,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Revealer",
        id: 14310201,
        monster_type: "MACHINE",
        level: 1,
        scale: 1.0,
        exp: 270,
        job: 387,
        hp: 960,
        sp: 0,
        mp: 0,
        def: [11, 15],
        mdef: [38, 30],
        properties: Properties {
            fire: 12,
            water: 12,
            wind: 12,
            earth: 12,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK Test No.10",
        id: 14310202,
        monster_type: "MACHINE",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Vant",
        id: 14310500,
        monster_type: "MACHINE",
        level: 98,
        scale: 1.0,
        exp: 29385,
        job: 27900,
        hp: 20000,
        sp: 0,
        mp: 0,
        def: [70, 400],
        mdef: [55, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037505, 10009900, 10019900, 10025452, 60071380, 60071380, 0,
        ],
    },
    Monster {
        name: "Vant",
        id: 14310501,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 3150,
        job: 4050,
        hp: 50000,
        sp: 0,
        mp: 0,
        def: [12, 22],
        mdef: [42, 40],
        properties: Properties {
            fire: 15,
            water: 15,
            wind: 15,
            earth: 15,
            light: 4,
            dark: 15,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Vant",
        id: 14310502,
        monster_type: "MACHINE",
        level: 1,
        scale: 1.0,
        exp: 900,
        job: 2700,
        hp: 1800,
        sp: 0,
        mp: 0,
        def: [14, 20],
        mdef: [40, 40],
        properties: Properties {
            fire: 14,
            water: 14,
            wind: 14,
            earth: 14,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Vant",
        id: 14310503,
        monster_type: "MACHINE",
        level: 48,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK Test No.11",
        id: 14310504,
        monster_type: "MACHINE",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Preserver",
        id: 14310600,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 1,
        scale: 1.2,
        exp: 4500,
        job: 11250,
        hp: 60000,
        sp: 0,
        mp: 0,
        def: [14, 32],
        mdef: [44, 45],
        properties: Properties {
            fire: 17,
            water: 17,
            wind: 17,
            earth: 17,
            light: 5,
            dark: 17,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Preserver",
        id: 14310601,
        monster_type: "MACHINE_SKILL",
        level: 1,
        scale: 1.1,
        exp: 1350,
        job: 3150,
        hp: 2160,
        sp: 0,
        mp: 0,
        def: [15, 30],
        mdef: [42, 45],
        properties: Properties {
            fire: 16,
            water: 16,
            wind: 16,
            earth: 16,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK Test No.12",
        id: 14310602,
        monster_type: "MACHINE_SKILL",
        level: 30,
        scale: 1.1,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Crimson Blood",
        id: 14311600,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 1,
        scale: 1.3,
        exp: 6750,
        job: 18000,
        hp: 70000,
        sp: 0,
        mp: 0,
        def: [18, 42],
        mdef: [50, 50],
        properties: Properties {
            fire: 18,
            water: 18,
            wind: 18,
            earth: 18,
            light: 6,
            dark: 18,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Crimson Blood",
        id: 14311601,
        monster_type: "MACHINE_SKILL",
        level: 1,
        scale: 1.1,
        exp: 1800,
        job: 3600,
        hp: 3080,
        sp: 0,
        mp: 0,
        def: [16, 40],
        mdef: [48, 50],
        properties: Properties {
            fire: 18,
            water: 18,
            wind: 18,
            earth: 18,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK Test No.13",
        id: 14311602,
        monster_type: "MACHINE_SKILL",
        level: 30,
        scale: 1.1,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM?|Klinge R",
        id: 14320000,
        monster_type: "HUMAN_SKILL",
        level: 99,
        scale: 1.0,
        exp: 29250,
        job: 29925,
        hp: 21500,
        sp: 0,
        mp: 0,
        def: [60, 260],
        mdef: [45, 180],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037505, 10009900, 10019900, 10025452, 60071380, 60071380, 0,
        ],
    },
    Monster {
        name: "DEM?|Klinge R",
        id: 14320001,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 3870,
        job: 5760,
        hp: 90000,
        sp: 0,
        mp: 0,
        def: [22, 110],
        mdef: [20, 60],
        properties: Properties {
            fire: 18,
            water: 18,
            wind: 32,
            earth: 18,
            light: 0,
            dark: 52,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Klinge R",
        id: 14320002,
        monster_type: "HUMAN_SKILL",
        level: 1,
        scale: 1.0,
        exp: 387,
        job: 576,
        hp: 2520,
        sp: 0,
        mp: 0,
        def: [20, 80],
        mdef: [16, 30],
        properties: Properties {
            fire: 8,
            water: 8,
            wind: 22,
            earth: 8,
            light: 0,
            dark: 42,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Klinge R",
        id: 14320003,
        monster_type: "HUMAN_SKILL",
        level: 49,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 01",
        id: 14320004,
        monster_type: "HUMAN_SKILL",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM?|Klinge R",
        id: 14320200,
        monster_type: "HUMAN",
        level: 97,
        scale: 1.0,
        exp: 28350,
        job: 29250,
        hp: 20550,
        sp: 0,
        mp: 0,
        def: [60, 260],
        mdef: [45, 180],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037505, 10009900, 10019900, 10025452, 60071380, 60071380, 0,
        ],
    },
    Monster {
        name: "DEM?|Klinger",
        id: 14320201,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 2835,
        job: 4455,
        hp: 70000,
        sp: 0,
        mp: 0,
        def: [14, 90],
        mdef: [16, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 5,
            earth: 0,
            light: 0,
            dark: 25,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Klinger",
        id: 14320202,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 285,
        job: 447,
        hp: 1320,
        sp: 0,
        mp: 0,
        def: [12, 60],
        mdef: [12, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 5,
            earth: 0,
            light: 0,
            dark: 25,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Klinger",
        id: 14320203,
        monster_type: "HUMAN",
        level: 47,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 02",
        id: 14320204,
        monster_type: "HUMAN",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM?|Gevain",
        id: 14320300,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 17100,
        job: 17505,
        hp: 100000,
        sp: 0,
        mp: 0,
        def: [26, 120],
        mdef: [20, 70],
        properties: Properties {
            fire: 20,
            water: 20,
            wind: 40,
            earth: 20,
            light: 0,
            dark: 60,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Gevain",
        id: 14320301,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 1710,
        job: 1752,
        hp: 3600,
        sp: 0,
        mp: 0,
        def: [24, 90],
        mdef: [16, 40],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 30,
            earth: 10,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM - AK Test 03",
        id: 14320302,
        monster_type: "HUMAN",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM?|Amajor",
        id: 14320500,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 10800,
        job: 12060,
        hp: 80000,
        sp: 0,
        mp: 0,
        def: [16, 105],
        mdef: [16, 50],
        properties: Properties {
            fire: 15,
            water: 15,
            wind: 25,
            earth: 15,
            light: 0,
            dark: 45,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Amajor",
        id: 14320501,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 1080,
        job: 1206,
        hp: 1800,
        sp: 0,
        mp: 0,
        def: [14, 75],
        mdef: [12, 20],
        properties: Properties {
            fire: 5,
            water: 5,
            wind: 15,
            earth: 5,
            light: 0,
            dark: 35,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Amajor ?A",
        id: 14320502,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 30,
        scale: 1.0,
        exp: 19800,
        job: 25560,
        hp: 80000,
        sp: 0,
        mp: 0,
        def: [20, 75],
        mdef: [10, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10067302, 0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM - AK Test 04",
        id: 14320503,
        monster_type: "HUMAN",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM-Gavea R",
        id: 14330000,
        monster_type: "HUMAN_SKILL",
        level: 100,
        scale: 1.0,
        exp: 30150,
        job: 30150,
        hp: 20515,
        sp: 0,
        mp: 0,
        def: [60, 250],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037505, 10009900, 10019900, 10025452, 60071380, 60071380, 0,
        ],
    },
    Monster {
        name: "DEM-Gavea R",
        id: 14330001,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 3690,
        job: 5940,
        hp: 80000,
        sp: 0,
        mp: 0,
        def: [12, 75],
        mdef: [26, 100],
        properties: Properties {
            fire: 28,
            water: 28,
            wind: 42,
            earth: 28,
            light: 0,
            dark: 62,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM-Gavea R",
        id: 14330002,
        monster_type: "HUMAN_SKILL",
        level: 1,
        scale: 1.0,
        exp: 369,
        job: 594,
        hp: 2160,
        sp: 0,
        mp: 0,
        def: [10, 45],
        mdef: [32, 90],
        properties: Properties {
            fire: 18,
            water: 18,
            wind: 32,
            earth: 18,
            light: 0,
            dark: 52,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM-Gavea R",
        id: 14330003,
        monster_type: "HUMAN_SKILL",
        level: 50,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 05",
        id: 14330004,
        monster_type: "HUMAN_SKILL",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM?|Smelts",
        id: 14330200,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.2,
        exp: 2880,
        job: 4590,
        hp: 66000,
        sp: 0,
        mp: 0,
        def: [10, 55],
        mdef: [22, 70],
        properties: Properties {
            fire: 20,
            water: 20,
            wind: 25,
            earth: 20,
            light: 0,
            dark: 45,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Smelts",
        id: 14330201,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.1,
        exp: 288,
        job: 459,
        hp: 1200,
        sp: 0,
        mp: 0,
        def: [8, 25],
        mdef: [26, 60],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 15,
            earth: 10,
            light: 0,
            dark: 35,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Smelts ?A",
        id: 14330202,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 30,
        scale: 1.0,
        exp: 18000,
        job: 23760,
        hp: 60000,
        sp: 0,
        mp: 0,
        def: [6, 30],
        mdef: [22, 48],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10067302, 0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM - AK Test 06",
        id: 14330203,
        monster_type: "HUMAN",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM?|Yeild",
        id: 14330300,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.2,
        exp: 17235,
        job: 18000,
        hp: 86000,
        sp: 0,
        mp: 0,
        def: [14, 90],
        mdef: [26, 110],
        properties: Properties {
            fire: 30,
            water: 30,
            wind: 50,
            earth: 40,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Yeild",
        id: 14330301,
        monster_type: "HUMAN_SKILL",
        level: 1,
        scale: 1.1,
        exp: 1725,
        job: 1800,
        hp: 2880,
        sp: 0,
        mp: 0,
        def: [12, 60],
        mdef: [36, 100],
        properties: Properties {
            fire: 20,
            water: 20,
            wind: 40,
            earth: 30,
            light: 0,
            dark: 70,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM - AK Test 07",
        id: 14330302,
        monster_type: "HUMAN_SKILL",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM-Gavea",
        id: 14330500,
        monster_type: "HUMAN",
        level: 98,
        scale: 1.0,
        exp: 29250,
        job: 28350,
        hp: 26890,
        sp: 0,
        mp: 0,
        def: [60, 250],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037505, 10009900, 10019900, 10025452, 60071380, 60071380, 0,
        ],
    },
    Monster {
        name: "DEM-Gavea",
        id: 14330501,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 11340,
        job: 13275,
        hp: 74000,
        sp: 0,
        mp: 0,
        def: [14, 65],
        mdef: [22, 80],
        properties: Properties {
            fire: 25,
            water: 25,
            wind: 35,
            earth: 25,
            light: 0,
            dark: 55,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM-Gavea",
        id: 14330502,
        monster_type: "HUMAN_SKILL",
        level: 1,
        scale: 1.0,
        exp: 1134,
        job: 1329,
        hp: 1440,
        sp: 0,
        mp: 0,
        def: [12, 35],
        mdef: [28, 80],
        properties: Properties {
            fire: 15,
            water: 15,
            wind: 25,
            earth: 15,
            light: 0,
            dark: 45,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM-Gevea",
        id: 14330503,
        monster_type: "HUMAN",
        level: 48,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 08",
        id: 14330504,
        monster_type: "HUMAN_SKILL",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Pig",
        id: 14350000,
        monster_type: "ANIMAL",
        level: 25,
        scale: 1.0,
        exp: 360,
        job: 261,
        hp: 760,
        sp: 100,
        mp: 100,
        def: [12, 55],
        mdef: [21, 49],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 22,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Halloween King",
        id: 14405000,
        monster_type: "UNDEAD_BOSS_CHAMP_BOMB_SKILL_NOTPTDROPRANGE",
        level: 1,
        scale: 2.0,
        exp: 2250,
        job: 3600,
        hp: 360000,
        sp: 0,
        mp: 0,
        def: [8, 34],
        mdef: [8, 20],
        properties: Properties {
            fire: 20,
            water: 20,
            wind: 20,
            earth: 20,
            light: 0,
            dark: 84,
        },
        drop_ids: [
            10064407, 10064407, 10064407, 10064407, 10064407, 10064407, 10064407, 0,
        ],
    },
    Monster {
        name: "Jack",
        id: 14405001,
        monster_type: "UNDEAD_BOSS_BOMB_SKILL",
        level: 1,
        scale: 1.5,
        exp: 2250,
        job: 3600,
        hp: 360000,
        sp: 0,
        mp: 0,
        def: [8, 34],
        mdef: [8, 20],
        properties: Properties {
            fire: 20,
            water: 20,
            wind: 20,
            earth: 20,
            light: 0,
            dark: 84,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DoomDoom",
        id: 14410000,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 90,
        scale: 2.5,
        exp: 540000,
        job: 675000,
        hp: 1200000,
        sp: 0,
        mp: 0,
        def: [85, 300],
        mdef: [70, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            29007101, 29007102, 29007103, 29007104, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dark Elephant",
        id: 14410001,
        monster_type: "ANIMAL",
        level: 70,
        scale: 1.0,
        exp: 103950,
        job: 44550,
        hp: 32000,
        sp: 0,
        mp: 0,
        def: [45, 220],
        mdef: [70, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dark Worrior",
        id: 14410010,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 99,
        scale: 2.5,
        exp: 1575000,
        job: 1800000,
        hp: 2400000,
        sp: 0,
        mp: 0,
        def: [85, 300],
        mdef: [70, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            10009700, 10050350, 29007200, 29007300, 29007400, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DoomDoom",
        id: 14410011,
        monster_type: "ANIMAL",
        level: 70,
        scale: 1.0,
        exp: 103950,
        job: 44550,
        hp: 32000,
        sp: 0,
        mp: 0,
        def: [45, 220],
        mdef: [70, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Luisa Shakespear",
        id: 14430000,
        monster_type: "HUMAN_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 9450,
        job: 10800,
        hp: 4400,
        sp: 250,
        mp: 250,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Symbol no. 1",
        id: 14450000,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 8000,
        sp: 0,
        mp: 0,
        def: [10, 0],
        mdef: [15, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Symbol no. 2",
        id: 14450001,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 8000,
        sp: 0,
        mp: 0,
        def: [10, 0],
        mdef: [15, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Symbol no. 3",
        id: 14450002,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 8000,
        sp: 0,
        mp: 0,
        def: [10, 0],
        mdef: [15, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Symbol no. 4",
        id: 14450003,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 8000,
        sp: 0,
        mp: 0,
        def: [10, 0],
        mdef: [15, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Symbol no. 5",
        id: 14450004,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 8000,
        sp: 0,
        mp: 0,
        def: [10, 0],
        mdef: [15, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Symbol no. 6",
        id: 14450005,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 8000,
        sp: 0,
        mp: 0,
        def: [10, 0],
        mdef: [15, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Symbol no. 7",
        id: 14450006,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 8000,
        sp: 0,
        mp: 0,
        def: [10, 0],
        mdef: [15, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK - Test S01",
        id: 14450007,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK - Test S02",
        id: 14450008,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK - Test S03",
        id: 14450009,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK - Test S04",
        id: 14450010,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK - Test S05",
        id: 14450011,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK - Test S06",
        id: 14450012,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK - Test S07",
        id: 14450013,
        monster_type: "MACHINE_SMARK_BOSS_SKILL_HETERODOXY_NONBLAST",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Symbol Wreckage",
        id: 14460000,
        monster_type: "ROCK_BOSS_SKILL_WALL",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 16000,
        sp: 0,
        mp: 0,
        def: [60, 0],
        mdef: [88, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "AK - Test S08",
        id: 14460001,
        monster_type: "ROCK_BOSS_SKILL_WALL",
        level: 1,
        scale: 1.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mysterious Boy",
        id: 14470000,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.0,
        exp: 27000,
        job: 40050,
        hp: 169999,
        sp: 999,
        mp: 999,
        def: [20, 100],
        mdef: [25, 80],
        properties: Properties {
            fire: 50,
            water: 50,
            wind: 100,
            earth: 50,
            light: 0,
            dark: 200,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mysterious Boy",
        id: 14470001,
        monster_type: "HUMAN_BOSS",
        level: 50,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mysterious Boy",
        id: 14470002,
        monster_type: "HUMAN_BOSS",
        level: 50,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mysterious Girl",
        id: 14480000,
        monster_type: "HUMAN_SKILL_BOSS_CHAMP",
        level: 1,
        scale: 1.0,
        exp: 27000,
        job: 40050,
        hp: 129999,
        sp: 999,
        mp: 999,
        def: [20, 80],
        mdef: [10, 30],
        properties: Properties {
            fire: 50,
            water: 25,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mysterious Girl",
        id: 14480001,
        monster_type: "HUMAN_BOSS",
        level: 15,
        scale: 1.0,
        exp: 675,
        job: 675,
        hp: 5000,
        sp: 999,
        mp: 999,
        def: [30, 140],
        mdef: [30, 160],
        properties: Properties {
            fire: 20,
            water: 20,
            wind: 20,
            earth: 20,
            light: 20,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mysterious Girl",
        id: 14480002,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Emil Dragon",
        id: 14520000,
        monster_type: "ANIMAL_BOSS_SKILL_NOTPTDROPRANGE",
        level: 113,
        scale: 1.5,
        exp: 954000,
        job: 517500,
        hp: 4500000,
        sp: 0,
        mp: 0,
        def: [70, 650],
        mdef: [70, 750],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Emil Dragon",
        id: 14520001,
        monster_type: "ANIMAL_BOSS_SKILL_NOTPTDROPRANGE",
        level: 250,
        scale: 1.5,
        exp: 15750000,
        job: 20250000,
        hp: 6500000,
        sp: 0,
        mp: 0,
        def: [93, 750],
        mdef: [75, 800],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Emil Alma",
        id: 14520002,
        monster_type: "ANIMAL_BOSS_SKILL_NOTPTDROPRANGE",
        level: 250,
        scale: 0.85,
        exp: 15750000,
        job: 20250000,
        hp: 6500000,
        sp: 0,
        mp: 0,
        def: [93, 750],
        mdef: [75, 800],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Cyclone",
        id: 14530000,
        monster_type: "MACHINE_NOTPTDROPRANGE",
        level: 105,
        scale: 1.0,
        exp: 31500,
        job: 32625,
        hp: 51580,
        sp: 0,
        mp: 0,
        def: [53, 450],
        mdef: [50, 190],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10070600, 0, /*null*/
            10030500, 10067200, 10070800, 10070700, 0,
        ],
    },
    Monster {
        name: "Death Schyte",
        id: 14530100,
        monster_type: "MACHINE_NOTPTDROPRANGE",
        level: 111,
        scale: 1.0,
        exp: 34020,
        job: 35325,
        hp: 62580,
        sp: 0,
        mp: 0,
        def: [35, 300],
        mdef: [50, 190],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10070600, 10018002, 10070700, 10067200, 10070800, 10070700, 0,
        ],
    },
    Monster {
        name: "Tiduce Butterfly",
        id: 14540000,
        monster_type: "INSECT_NOTPTDROPRANGE",
        level: 100,
        scale: 1.0,
        exp: 31410,
        job: 31500,
        hp: 46590,
        sp: 0,
        mp: 0,
        def: [55, 230],
        mdef: [45, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10070000, 10001150, 10001200, 10067100, 10070000, 10070100, 0,
        ],
    },
    Monster {
        name: "Teriable Butterfly",
        id: 14540100,
        monster_type: "INSECT_NOTPTDROPRANGE",
        level: 116,
        scale: 1.0,
        exp: 32706,
        job: 33210,
        hp: 52500,
        sp: 0,
        mp: 0,
        def: [18, 230],
        mdef: [45, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10070000, 10001150, 10001200, 0, /*null*/
            10067100, 10070100, 10070100, 0,
        ],
    },
    Monster {
        name: "Backing Tef",
        id: 14540101,
        monster_type: "ELEMENT_SKILL",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 0,
        mp: 0,
        def: [1, 1],
        mdef: [100, 9999],
        properties: Properties {
            fire: 100,
            water: 100,
            wind: 100,
            earth: 100,
            light: 100,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Shadow",
        id: 14550000,
        monster_type: "ELEMENT_NOTPTDROPRANGE",
        level: 145,
        scale: 1.5,
        exp: 460350,
        job: 691200,
        hp: 155950,
        sp: 0,
        mp: 0,
        def: [45, 250],
        mdef: [60, 300],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            10000604, 10070500, 10019900, 10000409, 90000045, 10070800, 10070800, 0,
        ],
    },
    Monster {
        name: "Reprovator",
        id: 14550001,
        monster_type: "ELEMENT_SKILL_BOSS",
        level: 1,
        scale: 3.0,
        exp: 0,
        job: 0,
        hp: 66666,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [20, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Apparition",
        id: 14550200,
        monster_type: "ELEMENT_NOTPTDROPRANGE",
        level: 150,
        scale: 1.5,
        exp: 2006100,
        job: 2245050,
        hp: 172590,
        sp: 0,
        mp: 0,
        def: [25, 300],
        mdef: [65, 400],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            10070500, 10000102, 10070500, 10000409, 90000044, 10070800, 10070800, 0,
        ],
    },
    Monster {
        name: "Phantom",
        id: 14550900,
        monster_type: "ELEMENT_NOTPTDROPRANGE",
        level: 160,
        scale: 1.5,
        exp: 2291850,
        job: 2448900,
        hp: 209586,
        sp: 0,
        mp: 0,
        def: [35, 250],
        mdef: [70, 500],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            10000604, 10070500, 10019900, 10000409, 90000046, 10070800, 10070800, 0,
        ],
    },
    Monster {
        name: "Dryad",
        id: 14560000,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 120,
        scale: 1.0,
        exp: 37575,
        job: 38520,
        hp: 71980,
        sp: 0,
        mp: 0,
        def: [30, 320],
        mdef: [35, 280],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10070200, 10004905, 10004905, 10067200, 10070800, 10070200, 0,
        ],
    },
    Monster {
        name: "Dryad(New Type)",
        id: 14561300,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 120,
        scale: 1.0,
        exp: 37575,
        job: 38520,
        hp: 71980,
        sp: 0,
        mp: 0,
        def: [30, 320],
        mdef: [35, 280],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10070200, 10004905, 10004905, 10067200, 10070800, 10070200, 0,
        ],
    },
    Monster {
        name: "Feeler Plant",
        id: 14560300,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 125,
        scale: 1.0,
        exp: 56385,
        job: 56880,
        hp: 75845,
        sp: 0,
        mp: 0,
        def: [25, 320],
        mdef: [35, 280],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10070200, 10004902, 10004902, 10067200, 10070200, 10070800, 0,
        ],
    },
    Monster {
        name: "Rejecter Anlash",
        id: 14560301,
        monster_type: "ELEMENT_SKILL_BOSS",
        level: 1,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 2400,
        sp: 0,
        mp: 0,
        def: [8, 20],
        mdef: [12, 60],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Alcid",
        id: 14560500,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 130,
        scale: 1.0,
        exp: 67095,
        job: 79740,
        hp: 81110,
        sp: 0,
        mp: 0,
        def: [25, 320],
        mdef: [35, 280],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10070200, 10070300, 10002807, 10067200, 10070200, 10070300, 0,
        ],
    },
    Monster {
        name: "Alrun",
        id: 14560900,
        monster_type: "PLANT_BOSS_SKILL_NOTPTDROPRANGE",
        level: 145,
        scale: 1.0,
        exp: 931050,
        job: 981000,
        hp: 85860,
        sp: 0,
        mp: 0,
        def: [25, 320],
        mdef: [35, 280],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10070200, 10070300, 10002807, 10067200, 10070300, 10016401, 0,
        ],
    },
    Monster {
        name: "RoboKnight",
        id: 14570000,
        monster_type: "UNDEAD",
        level: 136,
        scale: 1.0,
        exp: 466200,
        job: 524250,
        hp: 87890,
        sp: 0,
        mp: 0,
        def: [35, 345],
        mdef: [45, 260],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015709, 10001200, 10067100, 10067200, 10000604, 90000045, 0,
        ],
    },
    Monster {
        name: "Eclusion",
        id: 14570001,
        monster_type: "ELEMENT",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 240,
        sp: 0,
        mp: 0,
        def: [6, 32],
        mdef: [50, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Doom Night",
        id: 14570100,
        monster_type: "UNDEAD",
        level: 148,
        scale: 1.0,
        exp: 1544400,
        job: 1638900,
        hp: 95680,
        sp: 0,
        mp: 0,
        def: [35, 345],
        mdef: [45, 260],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015707, 10001200, 10067100, 10067200, 10000604, 90000045, 0,
        ],
    },
    Monster {
        name: "Baku Baku",
        id: 14580000,
        monster_type: "ANIMAL_NOTPTDROPRANGE",
        level: 144,
        scale: 1.0,
        exp: 995400,
        job: 995400,
        hp: 87890,
        sp: 0,
        mp: 0,
        def: [33, 333],
        mdef: [66, 66],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10069900, 10000311, 10006450, 10067100, 10069900, 10069900, 0,
        ],
    },
    Monster {
        name: "Dimension Baku Baku",
        id: 14580001,
        monster_type: "ELEMENT",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 300,
        sp: 0,
        mp: 0,
        def: [0, 18],
        mdef: [3, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Devil Baku Baku",
        id: 14580100,
        monster_type: "ANIMAL_NOTPTDROPRANGE",
        level: 152,
        scale: 1.0,
        exp: 1693350,
        job: 1693350,
        hp: 108590,
        sp: 0,
        mp: 0,
        def: [33, 333],
        mdef: [66, 66],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10000311, 10069900, 10006450, 10067100, 10069900, 10069900, 0,
        ],
    },
    Monster {
        name: "Acid Baku Baku",
        id: 14580200,
        monster_type: "ANIMAL_NOTPTDROPRANGE",
        level: 157,
        scale: 1.0,
        exp: 2181600,
        job: 1910250,
        hp: 135080,
        sp: 0,
        mp: 0,
        def: [33, 333],
        mdef: [66, 66],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10069800, 10069900, 10006403, 10067100, 10069800, 10069900, 0,
        ],
    },
    Monster {
        name: "Drummed Explosives",
        id: 14600000,
        monster_type: "ROCK_BOSS_SKILL_WALL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "WF Armada",
        id: 14630000,
        monster_type: "MACHINE_BOSS_SKILL",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mukard Diner",
        id: 14640000,
        monster_type: "INSECT",
        level: 41,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mukard Reater",
        id: 14640001,
        monster_type: "INSECT",
        level: 44,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK Test No.14",
        id: 14640002,
        monster_type: "INSECT_BOSS_CHAMP",
        level: 30,
        scale: 10.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Talpa",
        id: 14650000,
        monster_type: "ANIMAL",
        level: 40,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Dominion Toad",
        id: 14660000,
        monster_type: "WATER_ANIMAL",
        level: 37,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "King of Toads",
        id: 14660001,
        monster_type: "WATER_ANIMAL_BOSS",
        level: 45,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Metavora",
        id: 14670000,
        monster_type: "MAGIC_CREATURE",
        level: 47,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK Test No.15",
        id: 14670001,
        monster_type: "MAGIC_CREATURE_BOMB_SKILL",
        level: 30,
        scale: 0.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Airy Diner",
        id: 14680000,
        monster_type: "INSECT",
        level: 39,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Airy Reater",
        id: 14680001,
        monster_type: "INSECT",
        level: 43,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK Test No.16",
        id: 14680002,
        monster_type: "INSECT_SKILL",
        level: 30,
        scale: 0.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "AK Test No.17",
        id: 14680003,
        monster_type: "INSECT_BOSS_CHAMP",
        level: 30,
        scale: 8.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - Frugal",
        id: 14690000,
        monster_type: "MACHINE",
        level: 47,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 12",
        id: 14690001,
        monster_type: "MACHINE_SKILL",
        level: 30,
        scale: 0.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - Veruvian",
        id: 14700000,
        monster_type: "MACHINE",
        level: 48,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 13",
        id: 14700001,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 30,
        scale: 2.8,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM-Elephant",
        id: 14710000,
        monster_type: "MACHINE",
        level: 50,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 14",
        id: 14710001,
        monster_type: "MACHINE",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 15",
        id: 14710002,
        monster_type: "MACHINE_BOSS_CHAMP",
        level: 30,
        scale: 2.3,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM-Dragon",
        id: 14720000,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 150,
        scale: 0.8,
        exp: 405000,
        job: 398250,
        hp: 1000000,
        sp: 0,
        mp: 0,
        def: [60, 430],
        mdef: [55, 550],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM-Dragon",
        id: 14720001,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 50,
        scale: 0.8,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Heartlike",
        id: 14740000,
        monster_type: "HUMAN_BOSS_SKILL_CHAMP",
        level: 1,
        scale: 1.4,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test F01",
        id: 14820000,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test F02",
        id: 14830000,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test F03",
        id: 14840000,
        monster_type: "HUMAN_BOSS_SKILL_CHAMP",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test F04",
        id: 14850000,
        monster_type: "HUMAN_BOSS_SKILL_CHAMP",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test F05",
        id: 14860000,
        monster_type: "HUMAN_BOSS_SKILL_CHAMP",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test MM",
        id: 14870000,
        monster_type: "HUMAN",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test MF",
        id: 14880000,
        monster_type: "HUMAN",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "For Debug",
        id: 19000000,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "For Debug",
        id: 19010000,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Masha",
        id: 19010001,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tita",
        id: 19010002,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Lurie",
        id: 19010003,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tita",
        id: 19010004,
        monster_type: "HUMAN_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Emil",
        id: 19010005,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Titus ",
        id: 19010006,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Berial",
        id: 19010007,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Masha",
        id: 19010008,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tita",
        id: 19010009,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Lurie",
        id: 19010010,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Masha",
        id: 19010011,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tita",
        id: 19010012,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Lurie",
        id: 19010013,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Arc",
        id: 19010016,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 90,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 5000,
        sp: 150,
        mp: 150,
        def: [30, 130],
        mdef: [30, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ponytail Maid",
        id: 19010017,
        monster_type: "HUMAN_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 247500,
        job: 292500,
        hp: 5000,
        sp: 500,
        mp: 500,
        def: [60, 500],
        mdef: [60, 400],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Demon",
        id: 19010018,
        monster_type: "HUMAN_BOSS",
        level: 40,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 5600,
        sp: 0,
        mp: 0,
        def: [10, 80],
        mdef: [30, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000505, 10000505, 10000505, 10000505, 10000505, 10000505, 10000505, 0,
        ],
    },
    Monster {
        name: "Maid",
        id: 19010020,
        monster_type: "HUMAN_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 247500,
        job: 292500,
        hp: 5000,
        sp: 500,
        mp: 500,
        def: [60, 500],
        mdef: [60, 400],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Maid",
        id: 19010021,
        monster_type: "HUMAN_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 247500,
        job: 292500,
        hp: 5000,
        sp: 500,
        mp: 500,
        def: [60, 500],
        mdef: [60, 400],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Boy",
        id: 19010022,
        monster_type: "HUMAN_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 247500,
        job: 292500,
        hp: 5000,
        sp: 500,
        mp: 500,
        def: [60, 500],
        mdef: [60, 400],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Berial",
        id: 19010023,
        monster_type: "HUMAN_SMARK_HETERODOXY",
        level: 15,
        scale: 1.0,
        exp: 27000,
        job: 40050,
        hp: 3000,
        sp: 999,
        mp: 999,
        def: [30, 140],
        mdef: [30, 160],
        properties: Properties {
            fire: 20,
            water: 20,
            wind: 20,
            earth: 20,
            light: 20,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Drunken Boy",
        id: 19010024,
        monster_type: "UNDEAD_BOSS_SKILL_CHAMP",
        level: 1,
        scale: 1.0,
        exp: 2250,
        job: 3600,
        hp: 200000,
        sp: 0,
        mp: 0,
        def: [8, 34],
        mdef: [8, 20],
        properties: Properties {
            fire: 20,
            water: 20,
            wind: 20,
            earth: 20,
            light: 0,
            dark: 84,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Masha",
        id: 19010025,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tita",
        id: 19010026,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Lurie",
        id: 19010027,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mysterious DEM Girl",
        id: 19010028,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Overlooking Woman",
        id: 19010029,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Unidentified Body",
        id: 19010032,
        monster_type: "HUMAN_BOSS_MIRROR",
        level: 113,
        scale: 1.0,
        exp: 477000,
        job: 258750,
        hp: 1500000,
        sp: 1000,
        mp: 1000,
        def: [75, 650],
        mdef: [75, 800],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001200, 10000604, 10019900, 0, /*null*/
            10049000, 10009600, 10017900, 0,
        ],
    },
    Monster {
        name: "Unidentified Body",
        id: 19010033,
        monster_type: "HUMAN_BOSS_MIRROR",
        level: 113,
        scale: 1.0,
        exp: 477000,
        job: 258750,
        hp: 1500000,
        sp: 1000,
        mp: 1000,
        def: [75, 650],
        mdef: [75, 800],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001200, 10000604, 10019900, 0, /*null*/
            10049000, 10009600, 10017900, 0,
        ],
    },
    Monster {
        name: "Unidentified Body",
        id: 19010034,
        monster_type: "HUMAN_BOSS_MIRROR",
        level: 113,
        scale: 1.0,
        exp: 477000,
        job: 258750,
        hp: 1500000,
        sp: 1000,
        mp: 1000,
        def: [75, 650],
        mdef: [75, 800],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001200, 10000604, 10019900, 0, /*null*/
            10049000, 10009600, 10017900, 0,
        ],
    },
    Monster {
        name: "Emil",
        id: 19010035,
        monster_type: "HUMAN_SMARK_HETERODOXY",
        level: 30,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mysterious DEM Boy",
        id: 19010036,
        monster_type: "HUMAN_SMARK_HETERODOXY",
        level: 50,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "For Debug",
        id: 19020000,
        monster_type: "HUMAN_SKILL_CHAMP",
        level: 99,
        scale: 1.0,
        exp: 22500,
        job: 22500,
        hp: 5000,
        sp: 100,
        mp: 150,
        def: [26, 41],
        mdef: [24, 32],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002800, 10020500, 10019900, 10020550, 10020500, 10020550, 0,
        ],
    },
    Monster {
        name: "For Debug",
        id: 19030000,
        monster_type: "HUMAN_CHAMP",
        level: 18,
        scale: 1.0,
        exp: 249,
        job: 114,
        hp: 230,
        sp: 100,
        mp: 150,
        def: [26, 41],
        mdef: [24, 32],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002800, 10020500, 10019900, 10020550, 10020500, 10020550, 0,
        ],
    },
    Monster {
        name: "For Debug",
        id: 19040000,
        monster_type: "HUMAN_CHAMP",
        level: 18,
        scale: 1.0,
        exp: 249,
        job: 114,
        hp: 10000,
        sp: 100,
        mp: 150,
        def: [26, 41],
        mdef: [24, 32],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002800, 10020500, 10019900, 10020550, 10020500, 10020550, 0,
        ],
    },
    Monster {
        name: "For Debug",
        id: 19050000,
        monster_type: "HUMAN_CHAMP",
        level: 18,
        scale: 1.0,
        exp: 249,
        job: 114,
        hp: 230,
        sp: 100,
        mp: 150,
        def: [26, 41],
        mdef: [24, 32],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002800, 10020500, 10019900, 10020550, 10020500, 10020550, 0,
        ],
    },
    Monster {
        name: "Harpy",
        id: 19060000,
        monster_type: "BIRD",
        level: 90,
        scale: 1.0,
        exp: 14400,
        job: 14850,
        hp: 12500,
        sp: 200,
        mp: 250,
        def: [50, 230],
        mdef: [24, 130],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018204, 10037505, 10019900, 10024800, 10030300, 61040250, 0,
        ],
    },
    Monster {
        name: "Okypete",
        id: 10930001,
        monster_type: "BIRD",
        level: 95,
        scale: 1.0,
        exp: 20250,
        job: 20925,
        hp: 18500,
        sp: 200,
        mp: 250,
        def: [50, 245],
        mdef: [30, 140],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 5,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018204, 10037505, 10037505, 10037505, 10030300, 90000043, 0,
        ],
    },
    Monster {
        name: "Habib",
        id: 10931400,
        monster_type: "BIRD",
        level: 95,
        scale: 1.0,
        exp: 20250,
        job: 20925,
        hp: 18500,
        sp: 200,
        mp: 250,
        def: [50, 245],
        mdef: [30, 140],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 5,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10018204, 10037505, 10037505, 10037505, 10030300, 90000043, 0,
        ],
    },
    Monster {
        name: "DEM-01",
        id: 19070000,
        monster_type: "HUMAN",
        level: 92,
        scale: 1.0,
        exp: 18900,
        job: 20475,
        hp: 18500,
        sp: 150,
        mp: 150,
        def: [55, 250],
        mdef: [30, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10037505, 10009900, 10019900, 10025452, 60071380, 60071380, 0,
        ],
    },
    Monster {
        name: "DEM-00",
        id: 19070001,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 3,
        job: 3,
        hp: 150,
        sp: 100,
        mp: 150,
        def: [12, 20],
        mdef: [10, 15],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|00E2",
        id: 19070002,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 20,
        scale: 1.0,
        exp: 18000,
        job: 20250,
        hp: 60000,
        sp: 150,
        mp: 150,
        def: [25, 130],
        mdef: [30, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|00P0",
        id: 19070003,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 2700,
        job: 5400,
        hp: 70000,
        sp: 150,
        mp: 150,
        def: [24, 120],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|00P0",
        id: 19070004,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 270,
        job: 540,
        hp: 1500,
        sp: 150,
        mp: 150,
        def: [20, 60],
        mdef: [20, 60],
        properties: Properties {
            fire: 8,
            water: 0,
            wind: 8,
            earth: 0,
            light: 0,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|01",
        id: 19070005,
        monster_type: "HUMAN",
        level: 42,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM?|00",
        id: 19070006,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 0?X",
        id: 19070007,
        monster_type: "HUMAN",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Eliminator",
        id: 19070400,
        monster_type: "HUMAN",
        level: 96,
        scale: 1.0,
        exp: 21825,
        job: 22500,
        hp: 25500,
        sp: 150,
        mp: 150,
        def: [50, 260],
        mdef: [40, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10033350, 10030500, 10030000, 60023981, 10030000, 60023980, 0,
        ],
    },
    Monster {
        name: "Debug Mob",
        id: 19070401,
        monster_type: "HUMAN",
        level: 92,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 2000,
        sp: 150,
        mp: 150,
        def: [55, 250],
        mdef: [30, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|00S3",
        id: 19070402,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 35,
        scale: 1.0,
        exp: 21600,
        job: 22500,
        hp: 75000,
        sp: 150,
        mp: 150,
        def: [30, 150],
        mdef: [40, 130],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|00F1",
        id: 19070403,
        monster_type: "HUMAN_BOSS_CHAMP",
        level: 1,
        scale: 1.1,
        exp: 3240,
        job: 5940,
        hp: 65000,
        sp: 150,
        mp: 150,
        def: [10, 30],
        mdef: [24, 120],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 20,
            earth: 10,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            10020209, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10037800, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|00F1",
        id: 19070404,
        monster_type: "HUMAN_SKILL",
        level: 1,
        scale: 1.0,
        exp: 324,
        job: 594,
        hp: 1800,
        sp: 150,
        mp: 150,
        def: [18, 50],
        mdef: [20, 60],
        properties: Properties {
            fire: 8,
            water: 0,
            wind: 8,
            earth: 0,
            light: 0,
            dark: 14,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Eliminator",
        id: 19070405,
        monster_type: "HUMAN",
        level: 46,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 10",
        id: 19070406,
        monster_type: "HUMAN_SKILL",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM-Sniper",
        id: 19070500,
        monster_type: "HUMAN",
        level: 96,
        scale: 1.0,
        exp: 22500,
        job: 21150,
        hp: 24500,
        sp: 150,
        mp: 150,
        def: [40, 250],
        mdef: [40, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001550, 10033350, 10030500, 10030000, 10021000, 10030000, 10025454, 0,
        ],
    },
    Monster {
        name: "Debug Mob",
        id: 19070501,
        monster_type: "HUMAN",
        level: 92,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 2000,
        sp: 150,
        mp: 150,
        def: [55, 250],
        mdef: [30, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|00S0",
        id: 19070502,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 405,
        job: 504,
        hp: 1575,
        sp: 150,
        mp: 150,
        def: [12, 30],
        mdef: [80, 100],
        properties: Properties {
            fire: 8,
            water: 0,
            wind: 8,
            earth: 0,
            light: 0,
            dark: 16,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM?|Sniper",
        id: 19070503,
        monster_type: "HUMAN",
        level: 46,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "DEM - AK Test 11",
        id: 19070504,
        monster_type: "HUMAN",
        level: 30,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Land Walker",
        id: 19080000,
        monster_type: "HUMAN",
        level: 95,
        scale: 1.0,
        exp: 23850,
        job: 24750,
        hp: 33000,
        sp: 150,
        mp: 150,
        def: [60, 330],
        mdef: [40, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            10002807, 10009900, 10019900, 10043707, 10043707, 50062100, 60010700, 0,
        ],
    },
    Monster {
        name: "Deskate",
        id: 19080001,
        monster_type: "HUMAN",
        level: 80,
        scale: 0.8,
        exp: 20700,
        job: 21600,
        hp: 128000,
        sp: 150,
        mp: 150,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10067302, 10000102, 10000101, 10000501, 90000045, 10034504, 0,
        ],
    },
    Monster {
        name: "Demice",
        id: 19080900,
        monster_type: "HUMAN",
        level: 115,
        scale: 1.0,
        exp: 38835,
        job: 41085,
        hp: 66666,
        sp: 150,
        mp: 150,
        def: [50, 345],
        mdef: [37, 135],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000604, 10070400, 10021302, 10067100, 10070400, 10070800, 0,
        ],
    },
    Monster {
        name: "Pivot(2 ",
        id: 20160400,
        monster_type: "HUMAN",
        level: 115,
        scale: 1.0,
        exp: 38835,
        job: 41085,
        hp: 66666,
        sp: 150,
        mp: 150,
        def: [50, 345],
        mdef: [37, 135],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000604, 10070400, 10021302, 10067100, 10070400, 10070800, 0,
        ],
    },
    Monster {
        name: "Pivot(4",
        id: 20160500,
        monster_type: "HUMAN",
        level: 115,
        scale: 1.0,
        exp: 38835,
        job: 41085,
        hp: 66666,
        sp: 150,
        mp: 150,
        def: [50, 345],
        mdef: [37, 135],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000604, 10070400, 10021302, 10067100, 10070400, 10070800, 0,
        ],
    },
    Monster {
        name: "Pivot(3",
        id: 20160700,
        monster_type: "HUMAN",
        level: 115,
        scale: 1.0,
        exp: 38835,
        job: 41085,
        hp: 66666,
        sp: 150,
        mp: 150,
        def: [50, 345],
        mdef: [37, 135],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000604, 10070400, 10021302, 10067100, 10070400, 10070800, 0,
        ],
    },
    Monster {
        name: "Pivot(1",
        id: 20161400,
        monster_type: "HUMAN",
        level: 115,
        scale: 1.0,
        exp: 38835,
        job: 41085,
        hp: 66666,
        sp: 150,
        mp: 150,
        def: [50, 345],
        mdef: [37, 135],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000604, 10070400, 10021302, 10067100, 10070400, 10070800, 0,
        ],
    },
    Monster {
        name: "Booster",
        id: 20210000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Shield",
        id: 20360000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Devil",
        id: 20420000,
        monster_type: "MAGIC_CREATURE",
        level: 50,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 10000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Devil",
        id: 20426000,
        monster_type: "MAGIC_CREATURE",
        level: 50,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 10000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Briking RX1",
        id: 26000000,
        monster_type: "MACHINE",
        level: 26,
        scale: 1.0,
        exp: 384,
        job: 249,
        hp: 240,
        sp: 67,
        mp: 126,
        def: [10, 98],
        mdef: [10, 26],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10015500, 0, /*null*/
            10023501, 10018000, 0, /*null*/
            10030001, 10050939,
        ],
    },
    Monster {
        name: "Infinite Briking RX1",
        id: 26000002,
        monster_type: "MACHINE",
        level: 39,
        scale: 1.0,
        exp: 450,
        job: 270,
        hp: 1250,
        sp: 1000,
        mp: 1000,
        def: [10, 61],
        mdef: [35, 82],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Briking RX3",
        id: 26000003,
        monster_type: "MACHINE",
        level: 46,
        scale: 1.0,
        exp: 2553,
        job: 1440,
        hp: 820,
        sp: 100,
        mp: 100,
        def: [63, 75],
        mdef: [20, 43],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10023501, 0, /*null*/
            10023501, 10018000, 0, /*null*/
            10030001, 10050989,
        ],
    },
    Monster {
        name: "Infinite Briking RX1",
        id: 26000004,
        monster_type: "MACHINE",
        level: 41,
        scale: 1.0,
        exp: 450,
        job: 270,
        hp: 800,
        sp: 1000,
        mp: 1000,
        def: [10, 61],
        mdef: [35, 82],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010020, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking RX1",
        id: 26000005,
        monster_type: "MACHINE",
        level: 41,
        scale: 1.0,
        exp: 450,
        job: 270,
        hp: 800,
        sp: 1000,
        mp: 1000,
        def: [10, 61],
        mdef: [35, 82],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010021, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking RX1",
        id: 26000006,
        monster_type: "MACHINE",
        level: 41,
        scale: 1.0,
        exp: 450,
        job: 270,
        hp: 800,
        sp: 1000,
        mp: 1000,
        def: [10, 61],
        mdef: [35, 82],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010022, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking RX1",
        id: 26000007,
        monster_type: "MACHINE",
        level: 41,
        scale: 1.0,
        exp: 450,
        job: 270,
        hp: 800,
        sp: 1000,
        mp: 1000,
        def: [10, 61],
        mdef: [35, 82],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010023, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking RX1",
        id: 26000008,
        monster_type: "MACHINE",
        level: 41,
        scale: 1.0,
        exp: 450,
        job: 270,
        hp: 800,
        sp: 1000,
        mp: 1000,
        def: [10, 61],
        mdef: [35, 82],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010024, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking RX3",
        id: 26000009,
        monster_type: "MACHINE",
        level: 45,
        scale: 1.0,
        exp: 720,
        job: 720,
        hp: 900,
        sp: 100,
        mp: 100,
        def: [33, 61],
        mdef: [30, 43],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010025, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking RX3",
        id: 26000010,
        monster_type: "MACHINE",
        level: 45,
        scale: 1.0,
        exp: 720,
        job: 720,
        hp: 900,
        sp: 100,
        mp: 100,
        def: [33, 61],
        mdef: [30, 43],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010026, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking RX3",
        id: 26000011,
        monster_type: "MACHINE",
        level: 45,
        scale: 1.0,
        exp: 720,
        job: 720,
        hp: 900,
        sp: 100,
        mp: 100,
        def: [33, 61],
        mdef: [30, 43],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010027, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking RX3",
        id: 26000012,
        monster_type: "MACHINE",
        level: 45,
        scale: 1.0,
        exp: 720,
        job: 720,
        hp: 900,
        sp: 100,
        mp: 100,
        def: [33, 61],
        mdef: [30, 43],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010028, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Briking RX3",
        id: 26000013,
        monster_type: "MACHINE",
        level: 45,
        scale: 1.0,
        exp: 720,
        job: 720,
        hp: 900,
        sp: 100,
        mp: 100,
        def: [33, 61],
        mdef: [30, 43],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10010029, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Guard Robot",
        id: 26000014,
        monster_type: "MACHINE_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nymph",
        id: 26010000,
        monster_type: "ELEMENT_NOTOUCH",
        level: 26,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 250,
        sp: 150,
        mp: 150,
        def: [20, 30],
        mdef: [45, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sprite",
        id: 26010200,
        monster_type: "ELEMENT",
        level: 26,
        scale: 1.0,
        exp: 645,
        job: 2025,
        hp: 650,
        sp: 150,
        mp: 150,
        def: [10, 47],
        mdef: [10, 33],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10001905, 10019900, 0, /*null*/
            90000045, 10034800, 10034850, 0,
        ],
    },
    Monster {
        name: "Purple Nymph",
        id: 26010201,
        monster_type: "ELEMENT",
        level: 1,
        scale: 1.0,
        exp: 645,
        job: 2025,
        hp: 650,
        sp: 150,
        mp: 150,
        def: [10, 47],
        mdef: [10, 33],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Banshee",
        id: 26010400,
        monster_type: "ELEMENT",
        level: 26,
        scale: 1.0,
        exp: 540,
        job: 375,
        hp: 250,
        sp: 150,
        mp: 150,
        def: [20, 30],
        mdef: [27, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 10,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10033905, 10002202, 10049000, 10013002, 10013000, 0,
        ],
    },
    Monster {
        name: "Yellow Nymph",
        id: 26010700,
        monster_type: "ELEMENT",
        level: 26,
        scale: 1.0,
        exp: 339,
        job: 375,
        hp: 250,
        sp: 150,
        mp: 150,
        def: [20, 30],
        mdef: [45, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10033905, 10002202, 0, /*null*/
            10013002, 10013000, 0,
        ],
    },
    Monster {
        name: "Fury",
        id: 26010900,
        monster_type: "ELEMENT",
        level: 36,
        scale: 1.0,
        exp: 2385,
        job: 2250,
        hp: 1400,
        sp: 250,
        mp: 250,
        def: [10, 65],
        mdef: [50, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10033904, 10033905, 10000102, 10049000, 10013002, 10013000, 0,
        ],
    },
    Monster {
        name: "Poisonous Lizard",
        id: 26040000,
        monster_type: "WATER_ANIMAL",
        level: 35,
        scale: 1.0,
        exp: 1305,
        job: 945,
        hp: 450,
        sp: 110,
        mp: 230,
        def: [50, 40],
        mdef: [38, 22],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10035600, 10019900, 10020201, 10020201, 10013850, 10050937,
        ],
    },
    Monster {
        name: "Flame Spirit",
        id: 26040001,
        monster_type: "WATER_ANIMAL_NOTOUCH",
        level: 35,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 450,
        sp: 110,
        mp: 230,
        def: [50, 40],
        mdef: [38, 22],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Salamader",
        id: 26040002,
        monster_type: "WATER_ANIMAL_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 450,
        sp: 110,
        mp: 230,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Salamandra 2",
        id: 26040400,
        monster_type: "WATER_ANIMAL_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 450,
        sp: 110,
        mp: 230,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Salamandra 4",
        id: 26040500,
        monster_type: "WATER_ANIMAL_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 450,
        sp: 110,
        mp: 230,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Salamandra 3",
        id: 26040700,
        monster_type: "WATER_ANIMAL_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 450,
        sp: 110,
        mp: 230,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Partisan",
        id: 26040900,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL",
        level: 105,
        scale: 2.0,
        exp: 292500,
        job: 315000,
        hp: 655500,
        sp: 500,
        mp: 500,
        def: [60, 500],
        mdef: [50, 450],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            10022850, 10009500, 10013800, 10009500, 10020760, 10050300, 10020760, 0,
        ],
    },
    Monster {
        name: "Arsenal Keeper",
        id: 26040901,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL_NOTPTDROPRANGE",
        level: 105,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 655500,
        sp: 500,
        mp: 500,
        def: [60, 500],
        mdef: [50, 450],
        properties: Properties {
            fire: 30,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            10021600, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10021601, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Salamandra 1",
        id: 26041300,
        monster_type: "WATER_ANIMAL_NOTOUCH",
        level: 74,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 450,
        sp: 110,
        mp: 230,
        def: [0, 210],
        mdef: [40, 110],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Phoenix",
        id: 26050000,
        monster_type: "BIRD",
        level: 64,
        scale: 1.0,
        exp: 8100,
        job: 6945,
        hp: 2400,
        sp: 100,
        mp: 100,
        def: [10, 220],
        mdef: [80, 144],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10000401, 10018301, 10049000, 10011201, 10001101, 10050999,
        ],
    },
    Monster {
        name: "Infinite Corrider Phoenix",
        id: 26050001,
        monster_type: "BIRD_BOSS",
        level: 55,
        scale: 1.5,
        exp: 3600,
        job: 3795,
        hp: 3400,
        sp: 100,
        mp: 100,
        def: [10, 200],
        mdef: [65, 120],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Thunder King",
        id: 26050002,
        monster_type: "BIRD_SPBOSS_SKILL",
        level: 64,
        scale: 4.0,
        exp: 252000,
        job: 156600,
        hp: 210000,
        sp: 100,
        mp: 100,
        def: [85, 450],
        mdef: [90, 400],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10009500, 10050300, 10009600, 10009600, 10050350, 10050300, 10050350, 10051006,
        ],
    },
    Monster {
        name: "Phoenix",
        id: 26050003,
        monster_type: "BIRD",
        level: 64,
        scale: 1.0,
        exp: 8100,
        job: 6945,
        hp: 2400,
        sp: 100,
        mp: 100,
        def: [10, 220],
        mdef: [85, 144],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10000401, 10018301, 10049000, 10011201, 10001101, 0,
        ],
    },
    Monster {
        name: "Dark Homura",
        id: 26050004,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 100000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Thunder King",
        id: 26050005,
        monster_type: "BIRD_SPBOSS_SKILL",
        level: 64,
        scale: 4.0,
        exp: 252000,
        job: 156600,
        hp: 210000,
        sp: 100,
        mp: 100,
        def: [85, 450],
        mdef: [90, 400],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10051006,
        ],
    },
    Monster {
        name: "King Bird",
        id: 26050700,
        monster_type: "BIRD",
        level: 74,
        scale: 1.2,
        exp: 11025,
        job: 10125,
        hp: 5000,
        sp: 100,
        mp: 100,
        def: [10, 220],
        mdef: [30, 144],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006400, 10000401, 10018301, 10049000, 10011201, 10001101, 0,
        ],
    },
    Monster {
        name: "Mock",
        id: 26060000,
        monster_type: "ELEMENT",
        level: 25,
        scale: 1.0,
        exp: 270,
        job: 180,
        hp: 310,
        sp: 58,
        mp: 97,
        def: [8, 29],
        mdef: [35, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 40,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10023900, 10018000, 10038300, 10049000, 10038500, 10009600, 10050944,
        ],
    },
    Monster {
        name: "Flame Stone",
        id: 26060100,
        monster_type: "ELEMENT",
        level: 91,
        scale: 1.0,
        exp: 270,
        job: 180,
        hp: 10500,
        sp: 58,
        mp: 97,
        def: [10, 150],
        mdef: [35, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 40,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10016606, 10034507, 10016300, 10016401, 10012100, 60041751, 0,
        ],
    },
    Monster {
        name: "Ice Stone",
        id: 26060400,
        monster_type: "ELEMENT",
        level: 50,
        scale: 1.0,
        exp: 3150,
        job: 2250,
        hp: 2850,
        sp: 255,
        mp: 255,
        def: [55, 200],
        mdef: [35, 130],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10009150, 10033904, 90000045, 0, /*null*/
            10038500, 90000045, 0,
        ],
    },
    Monster {
        name: "Kodama",
        id: 26061000,
        monster_type: "ELEMENT",
        level: 77,
        scale: 1.0,
        exp: 11700,
        job: 9450,
        hp: 5500,
        sp: 300,
        mp: 300,
        def: [30, 100],
        mdef: [35, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10023900, 10018000, 90000043, 0, /*null*/
            10038500, 90000043, 0,
        ],
    },
    Monster {
        name: "White Mock",
        id: 26061400,
        monster_type: "ELEMENT",
        level: 25,
        scale: 1.0,
        exp: 270,
        job: 180,
        hp: 310,
        sp: 58,
        mp: 97,
        def: [8, 29],
        mdef: [35, 70],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 40,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10023900, 10018000, 10038300, 0, /*null*/
            10038500, 10009600, 0,
        ],
    },
    Monster {
        name: "Infinite Mock",
        id: 26060001,
        monster_type: "ELEMENT",
        level: 21,
        scale: 1.0,
        exp: 81,
        job: 18,
        hp: 469,
        sp: 1000,
        mp: 1000,
        def: [10, 50],
        mdef: [10, 34],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 10037501, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite Mock",
        id: 26060002,
        monster_type: "ELEMENT",
        level: 19,
        scale: 1.0,
        exp: 72,
        job: 9,
        hp: 350,
        sp: 1000,
        mp: 1000,
        def: [10, 46],
        mdef: [10, 56],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 15,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mokju",
        id: 26070000,
        monster_type: "ELEMENT",
        level: 24,
        scale: 1.0,
        exp: 276,
        job: 324,
        hp: 204,
        sp: 69,
        mp: 144,
        def: [30, 17],
        mdef: [44, 48],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020207, 10004700, 10005611, 10020307, 10020307, 10016601, 10050928,
        ],
    },
    Monster {
        name: "Mokju",
        id: 26070001,
        monster_type: "ELEMENT",
        level: 24,
        scale: 1.0,
        exp: 285,
        job: 321,
        hp: 210,
        sp: 69,
        mp: 144,
        def: [5, 10],
        mdef: [56, 16],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10019200, 10019200, 10019200, 10049000, 10019200, 10019200, 10050928,
        ],
    },
    Monster {
        name: "Infinite Mokju",
        id: 26070002,
        monster_type: "ELEMENT",
        level: 47,
        scale: 1.0,
        exp: 675,
        job: 675,
        hp: 980,
        sp: 69,
        mp: 144,
        def: [20, 120],
        mdef: [20, 124],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mandragora",
        id: 26070003,
        monster_type: "ELEMENT_NOTOUCH",
        level: 47,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 980,
        sp: 69,
        mp: 144,
        def: [20, 120],
        mdef: [20, 124],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Preventer",
        id: 26070004,
        monster_type: "ELEMENT",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 180,
        sp: 69,
        mp: 144,
        def: [5, 8],
        mdef: [56, 16],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nenou",
        id: 26070500,
        monster_type: "ELEMENT_BOSS_SKILL",
        level: 80,
        scale: 3.5,
        exp: 396000,
        job: 292500,
        hp: 855000,
        sp: 1000,
        mp: 1000,
        def: [45, 400],
        mdef: [85, 400],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10009500, 10050300, 10009600, 10005611, 10016401, 10019305, 50042150, 0,
        ],
    },
    Monster {
        name: "Nenou",
        id: 26070501,
        monster_type: "ELEMENT_BOSS_SKILL",
        level: 80,
        scale: 3.5,
        exp: 396000,
        job: 292500,
        hp: 855000,
        sp: 1000,
        mp: 1000,
        def: [45, 400],
        mdef: [85, 400],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10009500, 10050300, 10009600, 10019402, 10019406, 10019305, 50042150, 0,
        ],
    },
    Monster {
        name: "Morri",
        id: 26070900,
        monster_type: "ELEMENT",
        level: 40,
        scale: 1.0,
        exp: 2475,
        job: 2025,
        hp: 1500,
        sp: 500,
        mp: 500,
        def: [10, 80],
        mdef: [30, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 35,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016605, 10016800, 10030700, 10032809, 10049000, 10019200, 10019305, 0,
        ],
    },
    Monster {
        name: "Araune",
        id: 26071000,
        monster_type: "ELEMENT",
        level: 67,
        scale: 1.0,
        exp: 8100,
        job: 8325,
        hp: 4458,
        sp: 1000,
        mp: 1000,
        def: [55, 135],
        mdef: [60, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10016605, 10030700, 10001907, 0, /*null*/
            10019200, 10019305, 0,
        ],
    },
    Monster {
        name: "Crystal Urchin",
        id: 26090000,
        monster_type: "ELEMENT",
        level: 24,
        scale: 1.0,
        exp: 207,
        job: 405,
        hp: 288,
        sp: 113,
        mp: 299,
        def: [35, 56],
        mdef: [52, 60],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10009150, 10001907, 10049000, 10019350, 10019303, 10050929,
        ],
    },
    Monster {
        name: "Snow Crystal",
        id: 26090002,
        monster_type: "ELEMENT",
        level: 31,
        scale: 1.5,
        exp: 474,
        job: 1104,
        hp: 366,
        sp: 999,
        mp: 999,
        def: [31, 110],
        mdef: [75, 25],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002202, 0, /*null*/
            10019900, 10049000, 10009500, 10019303, 10050953,
        ],
    },
    Monster {
        name: "Infinite Crystal",
        id: 26090003,
        monster_type: "ELEMENT",
        level: 27,
        scale: 1.0,
        exp: 117,
        job: 45,
        hp: 772,
        sp: 1000,
        mp: 1000,
        def: [10, 55],
        mdef: [10, 68],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Ruby",
        id: 26090100,
        monster_type: "ELEMENT",
        level: 90,
        scale: 1.0,
        exp: 207,
        job: 405,
        hp: 12000,
        sp: 113,
        mp: 299,
        def: [50, 150],
        mdef: [60, 150],
        properties: Properties {
            fire: 20,
            water: 10,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10009150, 10001907, 10049000, 10019350, 10019303, 0,
        ],
    },
    Monster {
        name: "Blue Crystal ",
        id: 26090300,
        monster_type: "MAGIC_CREATURE",
        level: 33,
        scale: 1.0,
        exp: 6,
        job: 0,
        hp: 100,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            10001804, 10001907, 10009150, 0, /*null*/
            10019350, 10019303, 0,
        ],
    },
    Monster {
        name: "Platinum",
        id: 26090700,
        monster_type: "ELEMENT",
        level: 76,
        scale: 1.0,
        exp: 11250,
        job: 8775,
        hp: 4350,
        sp: 250,
        mp: 300,
        def: [10, 250],
        mdef: [20, 50],
        properties: Properties {
            fire: 0,
            water: 10,
            wind: 20,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001150, 10014854, 10001907, 10014852, 10015707, 10019303, 0,
        ],
    },
    Monster {
        name: "Onyx",
        id: 26090900,
        monster_type: "ELEMENT",
        level: 47,
        scale: 1.0,
        exp: 1845,
        job: 2229,
        hp: 1550,
        sp: 300,
        mp: 300,
        def: [20, 80],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 10,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            10001550, 10014854, 10001907, 10015709, 10015709, 10019303, 0,
        ],
    },
    Monster {
        name: "Frosty Crystal",
        id: 26091400,
        monster_type: "ELEMENT",
        level: 24,
        scale: 1.0,
        exp: 249,
        job: 495,
        hp: 330,
        sp: 113,
        mp: 299,
        def: [35, 56],
        mdef: [52, 60],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10009150, 10001907, 10049000, 10019350, 10019303, 0,
        ],
    },
    Monster {
        name: "Icy Cool",
        id: 26091401,
        monster_type: "ELEMENT",
        level: 44,
        scale: 1.2,
        exp: 2250,
        job: 2025,
        hp: 1400,
        sp: 113,
        mp: 299,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10009150, 10001907, 0, /*null*/
            10019350, 10019303, 0,
        ],
    },
    Monster {
        name: "Snakehead Fish",
        id: 26100000,
        monster_type: "WATER_ANIMAL",
        level: 33,
        scale: 1.0,
        exp: 1395,
        job: 855,
        hp: 512,
        sp: 90,
        mp: 219,
        def: [25, 48],
        mdef: [30, 98],
        properties: Properties {
            fire: 0,
            water: 40,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007400, 10008600, 10007450, 10014501, 10014501, 10019400, 10050949,
        ],
    },
    Monster {
        name: "Diaper Man!",
        id: 26100001,
        monster_type: "WATER_ANIMAL_BOSS_SKILL",
        level: 43,
        scale: 2.0,
        exp: 156150,
        job: 156150,
        hp: 75000,
        sp: 90,
        mp: 219,
        def: [55, 90],
        mdef: [35, 84],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10007400, 10003950, 10007450, 10019400, 10049500, 10020403, 10049500, 10051000,
        ],
    },
    Monster {
        name: "Infinite Snakehead Fish",
        id: 26100002,
        monster_type: "WATER_ANIMAL",
        level: 22,
        scale: 1.0,
        exp: 90,
        job: 18,
        hp: 378,
        sp: 1000,
        mp: 1000,
        def: [10, 43],
        mdef: [10, 40],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Sandfish",
        id: 26100003,
        monster_type: "WATER_ANIMAL",
        level: 39,
        scale: 1.2,
        exp: 8100,
        job: 8100,
        hp: 1600,
        sp: 90,
        mp: 219,
        def: [45, 110],
        mdef: [25, 89],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007400, 10008600, 10007450, 10014501, 10014501, 10019400, 0,
        ],
    },
    Monster {
        name: "Gold Insmouse",
        id: 26100004,
        monster_type: "WATER_ANIMAL",
        level: 40,
        scale: 1.2,
        exp: 13500,
        job: 13500,
        hp: 1000,
        sp: 58,
        mp: 97,
        def: [96, 300],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10008600, 10000207, 10015300, 10050300, 10016000, 10000207, 10016000, 0,
        ],
    },
    Monster {
        name: "Spring Insmouse",
        id: 26100005,
        monster_type: "WATER_ANIMAL_BOSS_SKILL",
        level: 20,
        scale: 1.2,
        exp: 9036,
        job: 9036,
        hp: 2008,
        sp: 0,
        mp: 0,
        def: [10, 14],
        mdef: [18, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10025055, 10025055, 31160400, 31160400, 31160400, 31160400, 31160400, 0,
        ],
    },
    Monster {
        name: "Spring Insmouse",
        id: 26100006,
        monster_type: "WATER_ANIMAL_BOSS",
        level: 20,
        scale: 1.2,
        exp: 9036,
        job: 9036,
        hp: 2008,
        sp: 0,
        mp: 0,
        def: [10, 14],
        mdef: [18, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10025055, 10025055, 31160400, 31160400, 31160400, 31160400, 31160400, 0,
        ],
    },
    Monster {
        name: "Spring Insmouse",
        id: 26100007,
        monster_type: "WATER_ANIMAL_BOSS",
        level: 61,
        scale: 1.2,
        exp: 9036,
        job: 9036,
        hp: 200008,
        sp: 0,
        mp: 0,
        def: [30, 200],
        mdef: [35, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10025055, 10025055, 31162300, 31162300, 31162300, 31162300, 31162300, 0,
        ],
    },
    Monster {
        name: "Spring Insmouse",
        id: 26100008,
        monster_type: "WATER_ANIMAL_BOSS",
        level: 81,
        scale: 1.2,
        exp: 9036,
        job: 9036,
        hp: 2000008,
        sp: 0,
        mp: 0,
        def: [60, 400],
        mdef: [60, 500],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10025055, 10025055, 31132200, 31132200, 31132200, 31132200, 31132200, 0,
        ],
    },
    Monster {
        name: "Insmouse",
        id: 26100009,
        monster_type: "WATER_ANIMAL_NOTOUCH",
        level: 40,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 10000,
        sp: 58,
        mp: 97,
        def: [99, 300],
        mdef: [99, 300],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Thief Insmouse",
        id: 26100010,
        monster_type: "WATER_ANIMAL_BOSS",
        level: 30,
        scale: 1.2,
        exp: 1575,
        job: 1575,
        hp: 10000,
        sp: 0,
        mp: 0,
        def: [12, 50],
        mdef: [24, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Santa Thief Insmouse",
        id: 26100011,
        monster_type: "WATER_ANIMAL_BOSS",
        level: 30,
        scale: 1.2,
        exp: 1575,
        job: 1575,
        hp: 10000,
        sp: 0,
        mp: 0,
        def: [12, 50],
        mdef: [24, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Diaper Man!",
        id: 26100012,
        monster_type: "WATER_ANIMAL_BOSS_SKILL",
        level: 43,
        scale: 2.0,
        exp: 156150,
        job: 156150,
        hp: 75000,
        sp: 90,
        mp: 219,
        def: [55, 90],
        mdef: [35, 84],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10051000,
        ],
    },
    Monster {
        name: "ZirconI",
        id: 26130000,
        monster_type: "MAGIC_CREATURE",
        level: 17,
        scale: 1.0,
        exp: 144,
        job: 216,
        hp: 222,
        sp: 119,
        mp: 322,
        def: [10, 10],
        mdef: [20, 80],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10019900, 10011501, 10019307, 10014500, 10011500, 10050940,
        ],
    },
    Monster {
        name: "ZirconII",
        id: 26130001,
        monster_type: "MAGIC_CREATURE",
        level: 34,
        scale: 1.0,
        exp: 810,
        job: 1440,
        hp: 440,
        sp: 999,
        mp: 999,
        def: [10, 70],
        mdef: [78, 53],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10019900, 10011501, 10019307, 10014500, 10011500, 10050965,
        ],
    },
    Monster {
        name: "Infinite Turquoise",
        id: 26130002,
        monster_type: "MAGIC_CREATURE",
        level: 16,
        scale: 1.0,
        exp: 36,
        job: 18,
        hp: 350,
        sp: 1000,
        mp: 1000,
        def: [10, 53],
        mdef: [10, 51],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 10,
            light: 10,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "ZirconIII",
        id: 26130003,
        monster_type: "MAGIC_CREATURE",
        level: 42,
        scale: 1.0,
        exp: 945,
        job: 1530,
        hp: 900,
        sp: 999,
        mp: 999,
        def: [10, 70],
        mdef: [78, 53],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10014500, 10011501, 90000045, 10014500, 10011500, 0,
        ],
    },
    Monster {
        name: "Ice Element",
        id: 26130004,
        monster_type: "MAGIC_CREATURE_SKILL",
        level: 52,
        scale: 0.8,
        exp: 3744,
        job: 3600,
        hp: 30000,
        sp: 1000,
        mp: 1000,
        def: [15, 0],
        mdef: [20, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10067302, 10019304, 10019900, 10011501, 0, /*null*/
            10014500, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Obsidian",
        id: 26130100,
        monster_type: "MAGIC_CREATURE",
        level: 60,
        scale: 2.0,
        exp: 8100,
        job: 3375,
        hp: 7550,
        sp: 420,
        mp: 420,
        def: [20, 115],
        mdef: [50, 175],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10019900, 10011501, 10067300, 10014500, 10011500, 0,
        ],
    },
    Monster {
        name: "Gloomy Jell",
        id: 26130101,
        monster_type: "MAGIC_CREATURE_NOTPTDROPRANGE",
        level: 28,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 350,
        sp: 420,
        mp: 420,
        def: [15, 10],
        mdef: [50, 175],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031161, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dark Element",
        id: 26130102,
        monster_type: "MAGIC_CREATURE_SKILL",
        level: 52,
        scale: 0.8,
        exp: 4050,
        job: 3600,
        hp: 36000,
        sp: 420,
        mp: 420,
        def: [15, 0],
        mdef: [20, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            10067302, 10019304, 10019900, 10011501, 0, /*null*/
            10014500, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dark Element Crime",
        id: 26130103,
        monster_type: "MAGIC_CREATURE_SKILL",
        level: 64,
        scale: 1.3,
        exp: 7425,
        job: 7425,
        hp: 52000,
        sp: 420,
        mp: 420,
        def: [15, 0],
        mdef: [20, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10067302, 10011501, 0, /*null*/
            10014500, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite Turquiose",
        id: 26130500,
        monster_type: "EVENT_BOSS",
        level: 0,
        scale: 3.0,
        exp: 0,
        job: 0,
        hp: 1500,
        sp: 10,
        mp: 10,
        def: [10, 45],
        mdef: [5, 16],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "CoRandom",
        id: 26130900,
        monster_type: "MAGIC_CREATURE",
        level: 54,
        scale: 1.0,
        exp: 2475,
        job: 3150,
        hp: 1950,
        sp: 322,
        mp: 322,
        def: [15, 120],
        mdef: [40, 175],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10001300, 10032811, 0, /*null*/
            10014500, 10011500, 0,
        ],
    },
    Monster {
        name: "Zircon(Red)",
        id: 26135200,
        monster_type: "MAGIC_CREATURE",
        level: 60,
        scale: 2.0,
        exp: 8100,
        job: 3375,
        hp: 7550,
        sp: 420,
        mp: 420,
        def: [20, 115],
        mdef: [50, 175],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10019900, 10011501, 0, /*null*/
            10014500, 10011500, 0,
        ],
    },
    Monster {
        name: "Zircon(Yellow)",
        id: 26135300,
        monster_type: "MAGIC_CREATURE",
        level: 60,
        scale: 2.0,
        exp: 8100,
        job: 3375,
        hp: 7550,
        sp: 420,
        mp: 420,
        def: [20, 115],
        mdef: [50, 175],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10019900, 10011501, 0, /*null*/
            10014500, 10011500, 0,
        ],
    },
    Monster {
        name: "Zircon(Green)",
        id: 26135400,
        monster_type: "MAGIC_CREATURE",
        level: 60,
        scale: 2.0,
        exp: 8100,
        job: 3375,
        hp: 7550,
        sp: 420,
        mp: 420,
        def: [20, 115],
        mdef: [50, 175],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10019900, 10011501, 0, /*null*/
            10014500, 10011500, 0,
        ],
    },
    Monster {
        name: "Zircon(White)",
        id: 26135500,
        monster_type: "MAGIC_CREATURE",
        level: 60,
        scale: 2.0,
        exp: 8100,
        job: 3375,
        hp: 7550,
        sp: 420,
        mp: 420,
        def: [20, 115],
        mdef: [50, 175],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            10019304, 10019900, 10011501, 0, /*null*/
            10014500, 10011500, 0,
        ],
    },
    Monster {
        name: "SaCris",
        id: 26160000,
        monster_type: "MACHINE",
        level: 0,
        scale: 1.0,
        exp: 6,
        job: 0,
        hp: 100,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 10,
            dark: 10,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Infinite SaCris",
        id: 26160001,
        monster_type: "MACHINE",
        level: 22,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 300,
        sp: 1000,
        mp: 1000,
        def: [10, 100],
        mdef: [10, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite SaCris",
        id: 26160002,
        monster_type: "MACHINE",
        level: 29,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 1000,
        mp: 1000,
        def: [10, 100],
        mdef: [10, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Infinite SaCris",
        id: 26160003,
        monster_type: "MACHINE",
        level: 24,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 1000,
        mp: 1000,
        def: [10, 100],
        mdef: [10, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mysterious Boy",
        id: 26160004,
        monster_type: "MACHINE",
        level: 40,
        scale: 1.5,
        exp: 15,
        job: 6,
        hp: 3500,
        sp: 300,
        mp: 300,
        def: [10, 50],
        mdef: [40, 20],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10054000, 10054000, 10054000, 10054000, 10054000, 10054000, 10054000, 0,
        ],
    },
    Monster {
        name: "D Pivot",
        id: 26160005,
        monster_type: "MACHINE",
        level: 85,
        scale: 2.0,
        exp: 9450,
        job: 9900,
        hp: 8500,
        sp: 300,
        mp: 300,
        def: [30, 150],
        mdef: [10, 20],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10030500, 10014800, 10019900, 10018004, 10011208, 10011208, 0,
        ],
    },
    Monster {
        name: "D Pivot",
        id: 26160006,
        monster_type: "MACHINE",
        level: 40,
        scale: 0.6,
        exp: 0,
        job: 0,
        hp: 100,
        sp: 300,
        mp: 300,
        def: [10, 50],
        mdef: [10, 20],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dumpty",
        id: 26180000,
        monster_type: "MAGIC_CREATURE",
        level: 37,
        scale: 1.0,
        exp: 1215,
        job: 1485,
        hp: 540,
        sp: 89,
        mp: 203,
        def: [22, 62],
        mdef: [65, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 30,
        },
        drop_ids: [
            0, /*null*/
            10002202, 10019900, 10020208, 10045901, 10020208, 10022000, 10050959,
        ],
    },
    Monster {
        name: "Broken Stuffed Doll",
        id: 26180001,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL",
        level: 43,
        scale: 0.5,
        exp: 135000,
        job: 169650,
        hp: 65000,
        sp: 89,
        mp: 203,
        def: [35, 160],
        mdef: [65, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            10002202, 10045902, 10020208, 10022000, 10049500, 10043400, 10049500, 10051001,
        ],
    },
    Monster {
        name: "Peekaboo",
        id: 26180002,
        monster_type: "MAGIC_CREATURE",
        level: 38,
        scale: 0.8,
        exp: 3960,
        job: 7200,
        hp: 1800,
        sp: 89,
        mp: 203,
        def: [1, 200],
        mdef: [30, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            0, /*null*/
            10002202, 10019900, 10020208, 10045901, 10020208, 10022000, 0,
        ],
    },
    Monster {
        name: "Broken Stuffed Doll",
        id: 26180003,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL",
        level: 43,
        scale: 0.5,
        exp: 135000,
        job: 169650,
        hp: 65000,
        sp: 89,
        mp: 203,
        def: [35, 160],
        mdef: [65, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            10002202, 10045902, 10020208, 10022000, 10049500, 10043400, 10049500, 10051001,
        ],
    },
    Monster {
        name: "Giant Tiny",
        id: 26180004,
        monster_type: "MAGIC_CREATURE",
        level: 50,
        scale: 3.0,
        exp: 225000,
        job: 157500,
        hp: 600000,
        sp: 89,
        mp: 203,
        def: [1, 200],
        mdef: [50, 90],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            10009500, 10009600, 10022151, 10022151, 0, 10009500, 10050350, 0,
        ],
    },
    Monster {
        name: "Giant Dumpty",
        id: 26180005,
        monster_type: "MAGIC_CREATURE",
        level: 50,
        scale: 3.0,
        exp: 180000,
        job: 135000,
        hp: 500000,
        sp: 89,
        mp: 203,
        def: [1, 200],
        mdef: [65, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            10009500, 10009600, 10022151, 10022151, 0, 10009500, 10009700, 0,
        ],
    },
    Monster {
        name: "Giant Peekaboo",
        id: 26180006,
        monster_type: "MAGIC_CREATURE",
        level: 50,
        scale: 3.0,
        exp: 180000,
        job: 135000,
        hp: 500000,
        sp: 89,
        mp: 203,
        def: [1, 200],
        mdef: [30, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            10009500, 10009600, 10022151, 10022151, 0, 10009500, 10009700, 0,
        ],
    },
    Monster {
        name: "Sand Bag(Bear)",
        id: 26180007,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 10,
        scale: 3.0,
        exp: 0,
        job: 0,
        hp: 600000,
        sp: 1,
        mp: 1,
        def: [50, 1],
        mdef: [50, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Joy Ride Dumpty",
        id: 26080008,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 50,
        scale: 3.0,
        exp: 180000,
        job: 135000,
        hp: 500000,
        sp: 89,
        mp: 203,
        def: [1, 200],
        mdef: [65, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            0, /*null*/
            10002202, 10019900, 10020208, 10045901, 10020208, 10022000, 0,
        ],
    },
    Monster {
        name: "Joy Ride Dumpty",
        id: 26080009,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 45,
        scale: 2.0,
        exp: 162000,
        job: 121500,
        hp: 400000,
        sp: 89,
        mp: 203,
        def: [0, 0],
        mdef: [30, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            0, /*null*/
            10002202, 10019900, 10020208, 10045901, 10020208, 10022000, 0,
        ],
    },
    Monster {
        name: "Small Joy Ride Dumpty",
        id: 26080010,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 43,
        scale: 0.8,
        exp: 145800,
        job: 109350,
        hp: 300000,
        sp: 89,
        mp: 203,
        def: [1, 200],
        mdef: [10, 13],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 40,
        },
        drop_ids: [
            0, /*null*/
            10002202, 10019900, 10020208, 10045901, 10020208, 10022000, 0,
        ],
    },
    Monster {
        name: "Naughty Bear",
        id: 26080011,
        monster_type: "MAGIC_CREATURE_SKILL",
        level: 24,
        scale: 0.5,
        exp: 0,
        job: 0,
        hp: 100,
        sp: 58,
        mp: 97,
        def: [95, 240],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            60082250, 60082251, 60082252, 60082252, 90000045, 90000046, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Very Naughty Bear",
        id: 26080012,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 70,
        scale: 3.5,
        exp: 180000,
        job: 180000,
        hp: 1000000,
        sp: 0,
        mp: 0,
        def: [1, 200],
        mdef: [65, 120],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002202, 10019900, 10020208, 10045901, 10020208, 10022000, 0,
        ],
    },
    Monster {
        name: "Broken Stuffed Doll",
        id: 26180013,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL",
        level: 43,
        scale: 0.5,
        exp: 135000,
        job: 169650,
        hp: 65000,
        sp: 89,
        mp: 203,
        def: [35, 160],
        mdef: [65, 110],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10051001,
        ],
    },
    Monster {
        name: "Tinyz 1",
        id: 26220000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Tinyz 2",
        id: 26230000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Tinyz 3",
        id: 26240000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Baby Tiny",
        id: 26240001,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Tiny 4",
        id: 26250000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Slave",
        id: 26260000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Dungeon TestIT MOB",
        id: 26270000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 100,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pet Pino",
        id: 26280000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 50,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pet Icy",
        id: 26290000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pet Electel",
        id: 26300000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 50,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pet Homura",
        id: 26310000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pet Tiny",
        id: 26320000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 700,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Live Staff",
        id: 26330000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 400,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Language Crystal",
        id: 26340000,
        monster_type: "MAGIC_CREATURE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10019901, 10019901, 10019901, 10019901, 10019901, 10019901, 10019901, 0,
        ],
    },
    Monster {
        name: "Language Crystal",
        id: 26340001,
        monster_type: "MAGIC_CREATURE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10019902, 10019902, 10019902, 10019902, 10019902, 10019902, 10019902, 0,
        ],
    },
    Monster {
        name: "Bamboo Barrel",
        id: 26350000,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 100000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Meman",
        id: 26390000,
        monster_type: "ELEMENT",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mermaid",
        id: 26390001,
        monster_type: "ELEMENT",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Butterfly",
        id: 90000000,
        monster_type: "INSECT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Butterfly",
        id: 90000001,
        monster_type: "INSECT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Butterfly",
        id: 90000050,
        monster_type: "INSECT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1200,
        sp: 1,
        mp: 1,
        def: [25, 10],
        mdef: [17, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Butterfly",
        id: 90000051,
        monster_type: "INSECT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1200,
        sp: 1,
        mp: 1,
        def: [25, 10],
        mdef: [17, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Black Butterfly",
        id: 90000052,
        monster_type: "INSECT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Alumof Spirit Butterfly",
        id: 90000053,
        monster_type: "INSECT_BOSS_NOTPTDROPRANGE",
        level: 90,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10031153, 10031153, 10031153, 10031153, 10031161, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Decoy Monster",
        id: 90010000,
        monster_type: "INSECT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mirror Explosion Monster",
        id: 90010001,
        monster_type: "INSECT_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010002,
        monster_type: "ROCK_MATERIAL_NORTH_NOTOUCH",
        level: 36,
        scale: 0.0,
        exp: 0,
        job: 0,
        hp: 444,
        sp: 130,
        mp: 367,
        def: [20, 90],
        mdef: [75, 95],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010003,
        monster_type: "ROCK_MATERIAL_SOUTH_NOTOUCH",
        level: 36,
        scale: 0.0,
        exp: 0,
        job: 0,
        hp: 444,
        sp: 130,
        mp: 367,
        def: [20, 90],
        mdef: [75, 95],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010004,
        monster_type: "ELEMENT_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010005,
        monster_type: "ELEMENT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010006,
        monster_type: "INSECT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Decoy Monster?Q",
        id: 90010007,
        monster_type: "INSECT",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010008,
        monster_type: "ELEMENT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1200,
        sp: 1,
        mp: 1,
        def: [50, 50],
        mdef: [50, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Trap ",
        id: 90010009,
        monster_type: "MACHINE_BOSS",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Decoy Monster?R",
        id: 90010010,
        monster_type: "MAGIC_CREATURE",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010011,
        monster_type: "MAGIC_CREATURE",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010012,
        monster_type: "MAGIC_CREATURE",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010013,
        monster_type: "MAGIC_CREATURE",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "0",
        id: 90010014,
        monster_type: "MAGIC_CREATURE",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "?@",
        id: 90010015,
        monster_type: "ELEMENT_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Decoy Monster4",
        id: 90010016,
        monster_type: "MAGIC_CREATURE",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Decoy Monster5",
        id: 90010017,
        monster_type: "MAGIC_CREATURE",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "?@",
        id: 90010018,
        monster_type: "ELEMENT_NOTOUCH_SKILL",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "?@",
        id: 90010019,
        monster_type: "ELEMENT_MATERIAL_NOTOUCH_SKILL",
        level: 3,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "?@",
        id: 90010020,
        monster_type: "ROCK_MATERIAL_NORTH_NOTOUCH",
        level: 36,
        scale: 0.0,
        exp: 0,
        job: 0,
        hp: 444,
        sp: 130,
        mp: 367,
        def: [20, 90],
        mdef: [75, 95],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "?@",
        id: 90010021,
        monster_type: "ROCK_MATERIAL_EAST_NOTOUCH",
        level: 36,
        scale: 0.0,
        exp: 0,
        job: 0,
        hp: 444,
        sp: 130,
        mp: 367,
        def: [20, 90],
        mdef: [75, 95],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "?@",
        id: 90010022,
        monster_type: "ROCK_MATERIAL_WEST_NOTOUCH",
        level: 36,
        scale: 0.0,
        exp: 0,
        job: 0,
        hp: 444,
        sp: 130,
        mp: 367,
        def: [20, 90],
        mdef: [75, 95],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "?@",
        id: 90010023,
        monster_type: "ROCK_MATERIAL_NORTH_NOTOUCH",
        level: 36,
        scale: 0.0,
        exp: 0,
        job: 0,
        hp: 444,
        sp: 130,
        mp: 367,
        def: [20, 90],
        mdef: [75, 95],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Rock",
        id: 30000000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [70, 1],
        mdef: [70, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014600, 10016900, 10014800, 10014650, 10014650, 10014851, 0,
        ],
    },
    Monster {
        name: "Ocean Rock",
        id: 30000001,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [70, 1],
        mdef: [70, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10014600, 10008400, 10014850, 10014850, 0, /*null*/
            90000046, 90000045, 0,
        ],
    },
    Monster {
        name: "Mountain Rock",
        id: 30000002,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [70, 1],
        mdef: [70, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014650, 10014750, 10022310, 0, /*null*/
            10015400, 90000043, 0,
        ],
    },
    Monster {
        name: "Heavy Rock",
        id: 30000003,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [70, 1],
        mdef: [70, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014853, 10014854, 10001300, 0, /*null*/
            90000044, 90000045, 0,
        ],
    },
    Monster {
        name: "South Rock",
        id: 30000004,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [70, 1],
        mdef: [70, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014600, 10014750, 10014854, 10015701, 10015701, 10015800, 0,
        ],
    },
    Monster {
        name: "Tiny Rock",
        id: 30000006,
        monster_type: "NONE_INFO_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 1,
        mp: 1,
        def: [40, 1],
        mdef: [40, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10014600, 10038001, 10019304, 10015600, 10015300, 10019650, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ancient Building Material",
        id: 30000007,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [70, 1],
        mdef: [70, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10047406, 10047407, 10047408, 10047408, 10047408, 10047408, 10047408, 0,
        ],
    },
    Monster {
        name: "Hard Rock",
        id: 30000008,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [70, 1],
        mdef: [70, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014853, 10014853, 10014853, 0, /*null*/
            10014650, 10014650, 0,
        ],
    },
    Monster {
        name: "Ordinary Rock",
        id: 30000009,
        monster_type: "ROCK_MATERIAL_BOSS_NOTPTDROPRANGE",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031156, 0, /*null*/
            0, /*null*/
            10031156, 10031156, 0,
        ],
    },
    Monster {
        name: "Ordinary Rock",
        id: 30000010,
        monster_type: "ROCK_MATERIAL_BOSS_NOTPTDROPRANGE",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 100,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031153, 10031153, 10031153, 10031153, 10031153, 0,
        ],
    },
    Monster {
        name: "Rage Rock",
        id: 30000011,
        monster_type: "ROCK_MATERIAL_BOSS_SKILL_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10031154, 0, /*null*/
            0, /*null*/
            0, /*null*/
            10031154, 10031154, 10031154, 0,
        ],
    },
    Monster {
        name: "Rock",
        id: 30000012,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Iron Ore Rock",
        id: 30010000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [80, 1],
        mdef: [80, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014853, 10014800, 10017000, 0, /*null*/
            10014851, 10014852, 0,
        ],
    },
    Monster {
        name: "Ore Deposite Rock",
        id: 30010001,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [80, 1],
        mdef: [80, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020209, 10014800, 10017000, 0, /*null*/
            10014800, 10014854, 0,
        ],
    },
    Monster {
        name: "Submarine Rock",
        id: 30010002,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [80, 1],
        mdef: [80, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10014600, 10014650, 10014600, 0, /*null*/
            10054800, 10046300, 10015400, 0,
        ],
    },
    Monster {
        name: "Tree Rock",
        id: 30010003,
        monster_type: "ROCK_MATERIAL_BOSS_NOTPTDROPRANGE",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10031158, 0, /*null*/
            0, /*null*/
            10031158, 10031158, 0,
        ],
    },
    Monster {
        name: "Iron Ore",
        id: 30010004,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014853, 10014851, 10014852, 0, /*null*/
            10015707, 10015709, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020001,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10046103, 10046100, 10046101, 0, /*null*/
            10046100, 10046102, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020002,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10046303, 10046300, 10046301, 0, /*null*/
            10046300, 10046302, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020003,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10046203, 10046200, 10046201, 0, /*null*/
            10046200, 10046202, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020004,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10046403, 10046400, 10046401, 0, /*null*/
            10046400, 10046402, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020005,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10046504, 10046502, 10046503, 0, /*null*/
            10046501, 10046500, 0,
        ],
    },
    Monster {
        name: "Miner Rock",
        id: 30020006,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10008400, 10014650, 10017300, 10015703, 10015703, 10015702, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020007,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10014600, 10014853, 10014851, 10014852, 10015707, 10015709, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020008,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10011201, 10046103, 10046100, 10046101, 10046100, 10046102, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020009,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10011203, 10046303, 10046300, 10046301, 10046300, 10046302, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020010,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10046203, 10046200, 10046201, 0, /*null*/
            10046200, 10046202, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020011,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10011205, 10046403, 10046400, 10046401, 10046400, 10046402, 0,
        ],
    },
    Monster {
        name: "Mineral Deposite",
        id: 30020012,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10011210, 10046504, 10046502, 10046503, 10046501, 10046500, 0,
        ],
    },
    Monster {
        name: "Fossil",
        id: 30020013,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10047424, 10047423, 10047423, 10047425, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Impure Rock",
        id: 30020014,
        monster_type: "ROCK_MATERIAL_BOSS_SKILL_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10031155, 0, /*null*/
            0, /*null*/
            0, /*null*/
            10031155, 10031155, 10031155, 0,
        ],
    },
    Monster {
        name: "Tragic Rock",
        id: 30020015,
        monster_type: "ROCK_MATERIAL_BOSS_SKILL_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10031159, 0, /*null*/
            0, /*null*/
            0, /*null*/
            10031159, 10031159, 10031159, 0,
        ],
    },
    Monster {
        name: "Fire Crystal",
        id: 30030000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [60, 1],
        mdef: [60, 0],
        properties: Properties {
            fire: 100,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10011201, 10046103, 10046100, 10011201, 10011201, 10046100, 0,
        ],
    },
    Monster {
        name: "Infinite Flame Crystal",
        id: 30030100,
        monster_type: "ROCK_MATERIAL",
        level: 55,
        scale: 1.0,
        exp: 675,
        job: 675,
        hp: 500,
        sp: 1,
        mp: 1,
        def: [10, 100],
        mdef: [50, 250],
        properties: Properties {
            fire: 100,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fire Crystal",
        id: 30030200,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [60, 1],
        mdef: [60, 0],
        properties: Properties {
            fire: 100,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10047410, 10047409, 10047411, 10047411, 10047411, 10047411, 10047411, 0,
        ],
    },
    Monster {
        name: "Water Crystal",
        id: 30030001,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [60, 1],
        mdef: [60, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10011203, 10046303, 0, /*null*/
            10011203, 10046300, 0,
        ],
    },
    Monster {
        name: "Earth Crystal",
        id: 30030002,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [60, 1],
        mdef: [60, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10011205, 10014500, 0, /*null*/
            10011205, 10046400, 0,
        ],
    },
    Monster {
        name: "Wind Crystal",
        id: 30030003,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [60, 1],
        mdef: [60, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 100,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10011207, 10014500, 0, /*null*/
            10011207, 10046200, 0,
        ],
    },
    Monster {
        name: "Infinite Wind Crystal",
        id: 30030700,
        monster_type: "ROCK_MATERIAL",
        level: 55,
        scale: 1.0,
        exp: 675,
        job: 675,
        hp: 500,
        sp: 1,
        mp: 1,
        def: [10, 100],
        mdef: [50, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 100,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Light Crystal",
        id: 30030004,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [60, 1],
        mdef: [60, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 100,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10011210, 10014500, 0, /*null*/
            10011210, 10011000, 0,
        ],
    },
    Monster {
        name: "Dark Crystal",
        id: 30030005,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [60, 1],
        mdef: [60, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10011209, 10014500, 0, /*null*/
            10011209, 10011000, 0,
        ],
    },
    Monster {
        name: "Crystalloid",
        id: 30030006,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [60, 1],
        mdef: [60, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10001800, 10019350, 10014500, 0, /*null*/
            10014300, 10011000, 0,
        ],
    },
    Monster {
        name: "Forest Crystal",
        id: 30030022,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [90, 1],
        mdef: [90, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000204, 10011205, 10000405, 10000304, 10013002, 10001907, 10013002, 0,
        ],
    },
    Monster {
        name: "Scorching Crystal",
        id: 30030026,
        monster_type: "ROCK_MATERIAL_BOSS_NOTPTDROPRANGE",
        level: 0,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 100,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            20050070, 0, /*null*/
            0, /*null*/
            0, /*null*/
            20050070, 20050070, 20050070, 0,
        ],
    },
    Monster {
        name: "Shiny Crystal",
        id: 30030027,
        monster_type: "ROCK_MATERIAL_BOSS_NOTPTDROPRANGE",
        level: 0,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 100,
            dark: 0,
        },
        drop_ids: [
            20050072, 0, /*null*/
            0, /*null*/
            0, /*null*/
            20050072, 20050072, 20050072, 0,
        ],
    },
    Monster {
        name: "Darkness Crystal",
        id: 30030028,
        monster_type: "ROCK_MATERIAL_BOSS_NOTPTDROPRANGE",
        level: 0,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 9999],
        mdef: [99, 9999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            20050071, 0, /*null*/
            0, /*null*/
            0, /*null*/
            20050071, 20050071, 20050071, 0,
        ],
    },
    Monster {
        name: "Infinite Crystalloid",
        id: 30030050,
        monster_type: "ROCK_MATERIAL",
        level: 53,
        scale: 1.0,
        exp: 675,
        job: 675,
        hp: 500,
        sp: 1,
        mp: 1,
        def: [10, 100],
        mdef: [25, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Maze Crystalloid",
        id: 30030051,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 1,
        mp: 1,
        def: [60, 1],
        mdef: [60, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Flame Sand Bag100",
        id: 30030007,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 100,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Water Sand Bag100",
        id: 30030008,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Earth Sand Bag100",
        id: 30030009,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wind Sand Bag100",
        id: 30030010,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 100,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Light Sand Bag100",
        id: 30030011,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 100,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dark Sand Bag100",
        id: 30030012,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Zero Sand Bag100",
        id: 30030013,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Water Sand Bag10",
        id: 30030014,
        monster_type: "ROCK",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 10,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Water Sand Bag20",
        id: 30030015,
        monster_type: "ROCK",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Water Sand Bag30",
        id: 30030016,
        monster_type: "ROCK",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Water Sand Bag40",
        id: 30030017,
        monster_type: "ROCK",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 40,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Water Sand Bag50",
        id: 30030018,
        monster_type: "ROCK",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Water Sand Bag60",
        id: 30030019,
        monster_type: "ROCK",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 60,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Water Sand Bag70",
        id: 30030020,
        monster_type: "ROCK",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 70,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Abnormal Sand Bag",
        id: 30030021,
        monster_type: "ROCK",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Multi Sand Bag",
        id: 30030023,
        monster_type: "PLANT_BOSS",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Only Attack Skill",
        id: 30030024,
        monster_type: "PLANT",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Only Magic Skill",
        id: 30030025,
        monster_type: "PLANT",
        level: 10,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sandback",
        id: 30030029,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 50,
            water: 50,
            wind: 50,
            earth: 50,
            light: 50,
            dark: 50,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEMstrate",
        id: 30030030,
        monster_type: "ROCK_MATERIAL_BOSS",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Wood Tree",
        id: 30040000,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10005610, 10028400, 10016300, 10016300, 10016300, 10016401, 0,
        ],
    },
    Monster {
        name: "Paper Tree",
        id: 30040001,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10020207, 10001905, 10020407, 10020307, 10020307, 10005611, 0,
        ],
    },
    Monster {
        name: "Eastern Fruit Tree",
        id: 30040002,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002800, 10003650, 10005612, 10052400, 10002900, 10002801, 0,
        ],
    },
    Monster {
        name: "Small Tree",
        id: 30040003,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016605, 10016605, 10016605, 10016605, 10016606, 10016606, 10016607, 0,
        ],
    },
    Monster {
        name: "Western Fruit Tree",
        id: 30040004,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003000, 10003650, 10005613, 10052400, 10003100, 10005611, 0,
        ],
    },
    Monster {
        name: "Southern Fruit Tree",
        id: 30040005,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003300, 10003650, 10005614, 10052400, 10003201, 10005611, 0,
        ],
    },
    Monster {
        name: "Northern Fruit Tree",
        id: 30040006,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003400, 10003650, 10005615, 10052400, 10003500, 10005611, 0,
        ],
    },
    Monster {
        name: "Tiny Tree",
        id: 30040007,
        monster_type: "NONE_INFO_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [30, 1],
        mdef: [30, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10002800, 10003000, 10003300, 10003400, 10003200, 10019650, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Orchard Tree",
        id: 30040008,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [30, 1],
        mdef: [30, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10002800, 10003000, 10003300, 10003400, 10033905, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nameless Tree",
        id: 30040009,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016605, 10020207, 10020207, 10020207, 10020207, 10016607, 10016601, 0,
        ],
    },
    Monster {
        name: "Strange Wood",
        id: 30040010,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 5,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [100, 1],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10005648, 10005648, 10005648, 10005648, 10005648, 0,
        ],
    },
    Monster {
        name: "Nymph Tree",
        id: 30050000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 3.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10016400, 10001907, 10034800, 10034850, 10016401, 10034506, 0,
        ],
    },
    Monster {
        name: "Wonder Tree",
        id: 30050001,
        monster_type: "PLANT_MATERIAL",
        level: 50,
        scale: 2.0,
        exp: 450000,
        job: 292500,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020202, 10015300, 10044102, 10015707, 10027100, 10016000, 10016000, 0,
        ],
    },
    Monster {
        name: "Wonder Tree",
        id: 30050002,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 3.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10059200, 10059200, 10059200, 10059200, 10059200, 10059200, 10059200, 0,
        ],
    },
    Monster {
        name: "Mistletoe Tree",
        id: 30050003,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 10,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [100, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fallen Tree",
        id: 30060000,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10016200, 10001905, 10028300, 10016400, 10016400, 10016350, 0,
        ],
    },
    Monster {
        name: "Ancient Tree",
        id: 30060001,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10016200, 10016200, 0, /*null*/
            10016350, 10061600, 10016401, 0,
        ],
    },
    Monster {
        name: "Driftwood",
        id: 30060002,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Morg Fossil Fuel",
        id: 30061000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10016700, 10016700, 0,
        ],
    },
    Monster {
        name: "Mushroom",
        id: 30070000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006900, 10007000, 10005616, 10006999, 10006999, 10007100, 0,
        ],
    },
    Monster {
        name: "Blue Mushroom",
        id: 30070003,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006900, 10006999, 10007001, 60044100, 10006900, 10007100, 0,
        ],
    },
    Monster {
        name: "Green Mushroom",
        id: 30070005,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006900, 10007002, 10007001, 60044105, 10006900, 60044105, 0,
        ],
    },
    Monster {
        name: "Orange Mushroom",
        id: 30070007,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006900, 10007000, 10007001, 60044100, 10006900, 60044100, 0,
        ],
    },
    Monster {
        name: "Brown Mushroom",
        id: 30070008,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006900, 10007003, 10007001, 60044100, 10006900, 10007100, 0,
        ],
    },
    Monster {
        name: "Black Mushroom",
        id: 30070009,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006900, 10007003, 10007001, 60107302, 10006900, 60044102, 0,
        ],
    },
    Monster {
        name: "Tiny Mushroom",
        id: 30070010,
        monster_type: "NONE_INFO_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [20, 1],
        mdef: [20, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10009300, 10009200, 10009450, 10034502, 10009111, 10019650, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Poison Mushroom",
        id: 30070011,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [20, 1],
        mdef: [20, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10007000, 10034502, 10005616, 10007100, 10007100, 10001109, 0,
        ],
    },
    Monster {
        name: "Chaos Mushroom",
        id: 30070012,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006900, 10006900, 10006900, 0, /*null*/
            10006999, 10006999, 0,
        ],
    },
    Monster {
        name: "Faint Mushroom",
        id: 30070013,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006900, 10006900, 10006900, 0, /*null*/
            10007000, 10007000, 0,
        ],
    },
    Monster {
        name: "Sweet Mushroom",
        id: 30070050,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10035400, 10007000, 10007000, 10007000, 10007000, 10007000, 0,
        ],
    },
    Monster {
        name: "Mom Mushroom",
        id: 30070051,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005633, 10005633, 10005633, 10005633, 10005639, 10005639, 0,
        ],
    },
    Monster {
        name: "Young Mushroom",
        id: 30070052,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005633, 10005633, 10005633, 10005633, 10005639, 10005639, 0,
        ],
    },
    Monster {
        name: "Mom Blue Mushroom",
        id: 30070053,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005634, 10005634, 10005634, 10005634, 10005640, 10005640, 0,
        ],
    },
    Monster {
        name: "Young Blue Mushroom",
        id: 30070054,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005634, 10005634, 10005634, 10005634, 10005640, 10005640, 0,
        ],
    },
    Monster {
        name: "Mom Green Mushroom",
        id: 30070055,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005635, 10005635, 10005635, 10005635, 10005641, 10005641, 0,
        ],
    },
    Monster {
        name: "Young Green Mushroom",
        id: 30070056,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005635, 10005635, 10005635, 10005635, 10005641, 10005641, 0,
        ],
    },
    Monster {
        name: "Mom Orange Mushroom",
        id: 30070057,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005636, 10005636, 10005636, 10005636, 10005642, 10005642, 0,
        ],
    },
    Monster {
        name: "Young Orange Mushroom",
        id: 30070058,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005636, 10005636, 10005636, 10005636, 10005642, 10005642, 0,
        ],
    },
    Monster {
        name: "Mom Brown Mushroom",
        id: 30070059,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005637, 10005637, 10005637, 10005637, 10005643, 10005643, 0,
        ],
    },
    Monster {
        name: "Young Brown Mushroom",
        id: 30070060,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005637, 10005637, 10005637, 10005637, 10005643, 10005643, 0,
        ],
    },
    Monster {
        name: "Mom Black Mushroom",
        id: 30070061,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005638, 10005638, 10005638, 10005638, 10005644, 10005644, 0,
        ],
    },
    Monster {
        name: "Young Black Mushroom",
        id: 30070062,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005638, 10005638, 10005638, 10005638, 10005644, 10005644, 0,
        ],
    },
    Monster {
        name: "Professor's Feast",
        id: 30070063,
        monster_type: "PLANT_MATERIAL_HETERODOXY",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 10,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Professor's Push",
        id: 30070064,
        monster_type: "PLANT_MATERIAL_HETERODOXY",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 10,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Breadfruit",
        id: 30110000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 1],
        mdef: [50, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006000, 10005601, 0, /*null*/
            10006200, 10006200, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nose Peanut",
        id: 30120000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 0],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10005700, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley",
        id: 30130000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005900, 10007900, 10005621, 10007900, 10007900, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley Grass",
        id: 30130001,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005900, 10005900, 10005900, 10005900, 10005900, 10005900, 0,
        ],
    },
    Monster {
        name: "Giant Barley Weed",
        id: 30130002,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley Weed",
        id: 30130003,
        monster_type: "PLANT MATERIAL_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10005622, 0, /*null*/
            10007900, 10005622, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley Weed (Large)",
        id: 30130004,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 3.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley Weed (Large)",
        id: 30130005,
        monster_type: "PLANT MATERIAL_SKILL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005623, 10007900, 0, /*null*/
            10007900, 10007900, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley Weed (Medium)",
        id: 30130006,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley Weed (Medium)",
        id: 30130007,
        monster_type: "PLANT MATERIAL_SKILL",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005624, 10007900, 0, /*null*/
            10007900, 10007900, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Silver Grass",
        id: 30130500,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10055402, 10055406, 10055405, 10055404, 10055403, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Herb",
        id: 30150000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10004901, 10003601, 10004903, 10005617, 10004903, 10004903, 10004903, 0,
        ],
    },
    Monster {
        name: "Herb",
        id: 30150001,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10004906, 10003601, 10004907, 10005618, 10004907, 10004907, 10004907, 0,
        ],
    },
    Monster {
        name: "Herb",
        id: 30150002,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10004905, 10003601, 10004908, 10005619, 10004908, 10004908, 10004908, 0,
        ],
    },
    Monster {
        name: "Herb",
        id: 30150003,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10004902, 10003601, 10004904, 10005620, 10004904, 10004904, 10004904, 0,
        ],
    },
    Monster {
        name: "Weed",
        id: 30150004,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10019202, 0, /*null*/
            10019202, 0,
        ],
    },
    Monster {
        name: "Weed",
        id: 30150005,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005632, 10005627, 10005629, 10019202, 10005631, 10019202, 0,
        ],
    },
    Monster {
        name: "Weed(Large)",
        id: 30150006,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10019202, 0, /*null*/
            10019202, 0,
        ],
    },
    Monster {
        name: "Weed(Large)",
        id: 30150007,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005625, 10005626, 10005628, 10019202, 10005630, 10019202, 0,
        ],
    },
    Monster {
        name: "Rainboots Herb",
        id: 30150008,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [20, 1],
        mdef: [20, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005632, 10005627, 10005629, 10019202, 10005631, 10019202, 0,
        ],
    },
    Monster {
        name: "Tiny Herb",
        id: 30150009,
        monster_type: "NONE_INFO_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [20, 1],
        mdef: [20, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10004000, 10034400, 10004902, 10004700, 10019601, 10019650, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Weed",
        id: 30150010,
        monster_type: "PLANT_MATERIAL_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005632, 10005627, 10005629, 10019202, 10005631, 10019202, 0,
        ],
    },
    Monster {
        name: "Steadily Grass",
        id: 30150011,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 252,
        job: 198,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10009105, 10009105, 10009105, 10009105, 10009105, 10009105, 10009105, 0,
        ],
    },
    Monster {
        name: "Steadily Grass",
        id: 30150012,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Bud of Flower(unsetting)",
        id: 30200000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10003412, 10003412, 10003412, 10003412, 10003412, 10003412, 10003412, 0,
        ],
    },
    Monster {
        name: "Flower",
        id: 30210000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005300, 0, /*null*/
            10005603, 10005400, 10005400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fire Flower",
        id: 30210100,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 100,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025205, 0, /*null*/
            10005604, 10043201, 10043201, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dark Flower",
        id: 30210200,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            10025205, 0, /*null*/
            10005609, 10043209, 10043209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ice Flower",
        id: 30210300,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025205, 0, /*null*/
            10005605, 10043203, 10043203, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tornado Flower",
        id: 30210700,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 100,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025205, 0, /*null*/
            10005606, 10043207, 10043207, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Earth Flower",
        id: 30210800,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025205, 0, /*null*/
            10005607, 10043208, 10043208, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Light Flower",
        id: 30211000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 100,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10004000, 0, /*null*/
            10005608, 10043204, 10043204, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tiny Flower",
        id: 30220000,
        monster_type: "NONE_INFO_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [20, 1],
        mdef: [20, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10035400, 10005603, 10005300, 10000408, 10005500, 10019650, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270000,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022101, 10022101, 0, /*null*/
            0, /*null*/
            10022101, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270001,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022102, 10022102, 0, /*null*/
            0, /*null*/
            10022102, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270002,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022103, 10022103, 0, /*null*/
            0, /*null*/
            10022103, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270003,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022104, 10022104, 0, /*null*/
            0, /*null*/
            10022104, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270004,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022105, 10022105, 0, /*null*/
            0, /*null*/
            10022105, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270006,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022106, 10022106, 0, /*null*/
            0, /*null*/
            10022106, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270007,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022107, 10022107, 0, /*null*/
            0, /*null*/
            10022107, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270008,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022108, 10022108, 0, /*null*/
            0, /*null*/
            10022108, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270013,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022113, 10022113, 0, /*null*/
            0, /*null*/
            10022113, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270014,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022114, 10022114, 0, /*null*/
            0, /*null*/
            10022114, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270015,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022114, 10022114, 0, /*null*/
            0, /*null*/
            10022114, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270016,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022116, 10022116, 0, /*null*/
            0, /*null*/
            10022116, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 30270017,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022117, 10022117, 0, /*null*/
            0, /*null*/
            10022117, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Container",
        id: 30280000,
        monster_type: "CONTAINER_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10022201, 10022201, 0, /*null*/
            0, /*null*/
            10022201, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Container",
        id: 30280001,
        monster_type: "CONTAINER_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10022202, 10022202, 0, /*null*/
            0, /*null*/
            10022202, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Container",
        id: 30280002,
        monster_type: "CONTAINER_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10022203, 10022203, 0, /*null*/
            0, /*null*/
            10022203, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Container",
        id: 30280003,
        monster_type: "CONTAINER_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10022204, 10022204, 0, /*null*/
            0, /*null*/
            10022204, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Container",
        id: 30280004,
        monster_type: "CONTAINER_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10022205, 10022205, 0, /*null*/
            0, /*null*/
            10022205, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Container",
        id: 30280006,
        monster_type: "CONTAINER_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10022206, 10022206, 0, /*null*/
            0, /*null*/
            10022206, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Container",
        id: 30280007,
        monster_type: "CONTAINER_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10022207, 10022207, 0, /*null*/
            0, /*null*/
            10022207, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Container",
        id: 30280008,
        monster_type: "CONTAINER_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 8,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10020209, 10022208, 10022208, 0, /*null*/
            0, /*null*/
            10022208, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290000,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022401, 10022401, 0, /*null*/
            0, /*null*/
            10022401, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290001,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022402, 10022402, 0, /*null*/
            0, /*null*/
            10022402, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290002,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022403, 10022403, 0, /*null*/
            0, /*null*/
            10022403, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290003,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022404, 10022404, 0, /*null*/
            0, /*null*/
            10022404, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290004,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022405, 10022405, 0, /*null*/
            0, /*null*/
            10022405, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290005,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022406, 10022406, 0, /*null*/
            0, /*null*/
            10022406, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290006,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022407, 10022407, 0, /*null*/
            0, /*null*/
            10022407, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290008,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022408, 10022408, 0, /*null*/
            0, /*null*/
            10022408, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290009,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022407, 10022407, 0, /*null*/
            0, /*null*/
            10022407, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290010,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022407, 10022407, 0, /*null*/
            0, /*null*/
            10022407, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290011,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10022412, 10022412, 0, /*null*/
            0, /*null*/
            10022412, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290012,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 30290013,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Wooden Box",
        id: 30290014,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Large Pot",
        id: 30300000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 1],
        mdef: [50, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001800, 10006000, 10009400, 10009000, 0, /*null*/
            10004000, 10006350, 0,
        ],
    },
    Monster {
        name: "Suspicious Pot",
        id: 30300001,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 1],
        mdef: [50, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10000103, 10000102, 10000108, 0, /*null*/
            10000503, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Old Pot",
        id: 30300002,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 1],
        mdef: [50, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10001550, 10001550, 10001550, 10001550, 10001550, 10001550, 10001550, 0,
        ],
    },
    Monster {
        name: "Old Pot",
        id: 30300003,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 1],
        mdef: [50, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10047419, 10047418, 10047418, 10047416, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Machinery Cluster",
        id: 30330000,
        monster_type: "MACHINE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10017100, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Brainwashing Machine",
        id: 30330001,
        monster_type: "MACHINE_MATERIAL",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 999,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10029700, 10029700, 10029700, 10029700, 10029700, 10029700, 10029700, 0,
        ],
    },
    Monster {
        name: "Machinery Cluster (Event)",
        id: 30330002,
        monster_type: "MACHINE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10057900, 10057900, 10057900, 10057900, 10057900, 10057900, 10057900, 0,
        ],
    },
    Monster {
        name: "Machine Cluster",
        id: 30330003,
        monster_type: "MACHINE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10047400, 10047401, 10047403, 10047403, 10047403, 10047403, 10047402, 0,
        ],
    },
    Monster {
        name: "Mechanical Wreckage",
        id: 30330004,
        monster_type: "MACHINE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10047404, 10047403, 10047405, 10047405, 10047405, 10047405, 10047405, 0,
        ],
    },
    Monster {
        name: "Defense System Wreckage",
        id: 30330005,
        monster_type: "MACHINE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10047417, 10047417, 10047417, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ancient Relic",
        id: 30330006,
        monster_type: "MACHINE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10047421, 10047420, 10047420, 10047422, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Machinery Cluster",
        id: 30330007,
        monster_type: "MACHINE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Machinery Cluster",
        id: 30330008,
        monster_type: "MACHINE_MATERIAL",
        level: 1,
        scale: 1.2,
        exp: 0,
        job: 0,
        hp: 9999999,
        sp: 9999999,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Volcanic Bomb",
        id: 30350000,
        monster_type: "ROCK_MATERIAL_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 100,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10001550, 10011308, 0,
        ],
    },
    Monster {
        name: "Cotton Plant",
        id: 30360000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10019600, 10019800, 10005602, 10019601, 10019601, 10024001, 0,
        ],
    },
    Monster {
        name: "Cotton Herb",
        id: 30360001,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10019600, 10019600, 10019600, 10019600, 10019600, 0,
        ],
    },
    Monster {
        name: "Giant Barley",
        id: 30390000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005900, 10007900, 10005621, 10007900, 10007900, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Stone Wall",
        id: 30470000,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 600,
        sp: 1,
        mp: 1,
        def: [0, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005300, 0, /*null*/
            10005603, 10005400, 10005400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Stone Wall",
        id: 30470001,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 500,
        sp: 1,
        mp: 1,
        def: [10, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005300, 0, /*null*/
            10005603, 10005400, 10005400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Stone Wall",
        id: 30470002,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 300,
        sp: 1,
        mp: 1,
        def: [50, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005300, 0, /*null*/
            10005603, 10005400, 10005400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Stone Wall",
        id: 30470003,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 100,
        sp: 1,
        mp: 1,
        def: [80, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005300, 0, /*null*/
            10005603, 10005400, 10005400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Stone Wall",
        id: 30470004,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 10,
        sp: 1,
        mp: 1,
        def: [100, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005300, 0, /*null*/
            10005603, 10005400, 10005400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tree Fairy's Healing Touch",
        id: 30480000,
        monster_type: "TREE_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 3498,
        job: 3498,
        hp: 777,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Elisabeth Terror",
        id: 30480001,
        monster_type: "PLANT_MATERIAL_BOSS_MARK",
        level: 0,
        scale: 5.0,
        exp: 0,
        job: 0,
        hp: 150,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [100, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Jumbo ChoockKo Statue",
        id: 30490000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 3498,
        job: 3498,
        hp: 7777,
        sp: 1,
        mp: 1,
        def: [77, 0],
        mdef: [88, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "BrikingMkII Statue",
        id: 30500000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 3498,
        job: 3498,
        hp: 7777,
        sp: 1,
        mp: 1,
        def: [77, 0],
        mdef: [88, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "King Roper Statue",
        id: 30510000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 3498,
        job: 3498,
        hp: 7777,
        sp: 1,
        mp: 1,
        def: [77, 0],
        mdef: [88, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Stinger Statue",
        id: 30520000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 3498,
        job: 3498,
        hp: 7777,
        sp: 1,
        mp: 1,
        def: [77, 0],
        mdef: [88, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "White Bear Statue",
        id: 30530000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 3498,
        job: 3498,
        hp: 7777,
        sp: 1,
        mp: 1,
        def: [77, 0],
        mdef: [88, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Pea",
        id: 30540000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 50],
        mdef: [50, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10005800, 10005800, 0, /*null*/
            10005800, 10016500, 0,
        ],
    },
    Monster {
        name: "Herbal Spice",
        id: 30550000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 50],
        mdef: [50, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10048100, 10053200, 0, /*null*/
            0, /*null*/
            10008500, 10008700, 0,
        ],
    },
    Monster {
        name: "Shining Herb",
        id: 30550700,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 50],
        mdef: [50, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10013107, 10013102, 10013103, 10013104, 10013106, 10013106, 0, 0,
        ],
    },
    Monster {
        name: "Gift Box",
        id: 30560000,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 4,
        sp: 1,
        mp: 1,
        def: [99, 1],
        mdef: [99, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Kadomazu",
        id: 30570000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Kagami Mochi",
        id: 30580000,
        monster_type: "NONE_INFO_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10008200, 10003300, 10008200, 10008200, 10008200, 31162000, 10008200, 0,
        ],
    },
    Monster {
        name: "Top",
        id: 30590000,
        monster_type: "PLANT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 10,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Top",
        id: 30590003,
        monster_type: "PLANT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 10,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Otsukimi Dango",
        id: 30600000,
        monster_type: "NONE_INFO_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10055401, 10055402, 10055403, 10055404, 10055405, 10055402, 10055406, 0,
        ],
    },
    Monster {
        name: "Reel",
        id: 30610000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DebugD1",
        id: 30610001,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Reel",
        id: 30620000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DebugD2",
        id: 30620001,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Reel",
        id: 30630000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DebugD3",
        id: 30630001,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Reel",
        id: 30640000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Reel",
        id: 30650000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Baobab Tree",
        id: 30680000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [80, 100],
        mdef: [80, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10016300, 10016400, 10028300, 10028400, 10019100, 10016350, 0,
        ],
    },
    Monster {
        name: "Baobab Tree",
        id: 30680001,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [80, 100],
        mdef: [80, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10016300, 10016400, 10028300, 10028400, 10019100, 10016350, 0,
        ],
    },
    Monster {
        name: "Device Control",
        id: 30690000,
        monster_type: "MACHINE_BOSS_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 15000,
        sp: 1,
        mp: 1,
        def: [70, 100],
        mdef: [50, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Device Control",
        id: 30690001,
        monster_type: "MACHINE_BOSS_SKILL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 15000,
        sp: 1,
        mp: 1,
        def: [70, 100],
        mdef: [50, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Healer Pigeon Clock",
        id: 30720000,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 100000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tiny Angel",
        id: 30730000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Watermelon",
        id: 30740000,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 9999,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10004350, 10004350, 10004350, 10004350, 10004350, 10004350, 10004350, 0,
        ],
    },
    Monster {
        name: "Weeping Willow",
        id: 30750000,
        monster_type: "PLANT_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 100000,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Misfortune Piece",
        id: 30760000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 6,
        sp: 1,
        mp: 1,
        def: [99, 100],
        mdef: [99, 100],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011308, 0,
        ],
    },
    Monster {
        name: "Living Dead",
        id: 30760100,
        monster_type: "UNDEAD_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [10, 30],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Quiet Spirit",
        id: 30760101,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [10, 30],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Elder Scarecow",
        id: 30760102,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [10, 30],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Bishop",
        id: 30760103,
        monster_type: "MAGIC_CREATURE_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [10, 30],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Strong Corpse",
        id: 30760104,
        monster_type: "UNDEAD_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [10, 30],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ghost",
        id: 30760105,
        monster_type: "UNDEAD_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [10, 30],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Specter",
        id: 30760106,
        monster_type: "HUMAN_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [10, 30],
        mdef: [10, 30],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Jack O Lantern",
        id: 30770000,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Jack O Lantern",
        id: 30770100,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Jack O Lantern",
        id: 30770200,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Jack O Lantern",
        id: 30770300,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Edible Pumpkin",
        id: 30770301,
        monster_type: "PLANT_MATERIAL_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10064407, 0,
        ],
    },
    Monster {
        name: "Jack O Lantern",
        id: 30770400,
        monster_type: "PLANT_NOTPTDROPRANGE",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 99999,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Palm Tree",
        id: 30780000,
        monster_type: "PLANT_MATERIAL_NOTPTDROPRANGE",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Angry Plant",
        id: 30790000,
        monster_type: "PLANT_NOTOUCH",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Angry Plant",
        id: 30800000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Barista",
        id: 30810000,
        monster_type: "PLANT_NOTOUCH",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Test",
        id: 30820000,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 0,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    Monster {
        name: "Steering Gear",
        id: 35000000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 50],
        mdef: [50, 50],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            10005800, 10005800, 0, /*null*/
            10005800, 10016500, 0,
        ],
    },
    Monster {
        name: "Live Door 1",
        id: 35030000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Live Door 2",
        id: 35040000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Live Door 3",
        id: 35050000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Live Door 4",
        id: 35060000,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Biological Door",
        id: 35060001,
        monster_type: "PLANT_MATERIAL_EAST_BOSS_SKILL_WALL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 15000,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Biological Door",
        id: 35060002,
        monster_type: "PLANT_MATERIAL_WEST_BOSS_SKILL_WALL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 15000,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Biological Door",
        id: 35060003,
        monster_type: "PLANT_MATERIAL_SOUTH_BOSS_SKILL_WALL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 15000,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Biological Door",
        id: 35060004,
        monster_type: "PLANT_MATERIAL_NORTH_BOSS_SKILL_WALL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 15000,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "PLANT",
        id: 10000001,
        monster_type: "PLANT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 1,
        mp: 1,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "BIRD",
        id: 10010001,
        monster_type: "BIRD",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 82,
        sp: 1,
        mp: 1,
        def: [5, 5],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10013100, 10034150, 0,
        ],
    },
    Monster {
        name: "ANIMAL",
        id: 10030001,
        monster_type: "ANIMAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005800, 10005800, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "INSECT",
        id: 10040001,
        monster_type: "INSECT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1000,
        sp: 1,
        mp: 1,
        def: [5, 5],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10020500, 10020550, 0,
        ],
    },
    Monster {
        name: "MAGIC_CREATURE",
        id: 10180001,
        monster_type: "MAGIC_CREATURE",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10003950, 10007450, 0,
        ],
    },
    Monster {
        name: "MACHINE",
        id: 26000001,
        monster_type: "MACHINE",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10003950, 10007450, 0,
        ],
    },
    Monster {
        name: "ROCK",
        id: 30000005,
        monster_type: "ROCK",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10014850, 10014851, 0,
        ],
    },
    Monster {
        name: "TREASURE_BOX",
        id: 30270005,
        monster_type: "TREASURE_BOX",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10022101, 10016401, 0,
        ],
    },
    Monster {
        name: "CONTAINER",
        id: 30280005,
        monster_type: "CONTAINER",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 1,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10022205, 10016401, 0,
        ],
    },
    Monster {
        name: "ELEMENT",
        id: 26090001,
        monster_type: "ELEMENT",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10022205, 10016401, 0,
        ],
    },
    Monster {
        name: "UNDEAD",
        id: 10200001,
        monster_type: "UNDEAD",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10022205, 10016401, 0,
        ],
    },
    Monster {
        name: "HUMAN",
        id: 26180009,
        monster_type: "HUMAN",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 30,
        sp: 1,
        mp: 1,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10032800, 10032800, 10032800, 10032800, 0, /*null*/
            10022205, 10016401, 0,
        ],
    },
    Monster {
        name: "ThisIsBOSS",
        id: 10000005,
        monster_type: "EVENT_BOSS",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Breadfruit",
        id: 40000001,
        monster_type: "CULTURE_TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [50, 1],
        mdef: [50, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006000, 10005601, 0, /*null*/
            10006200, 10006200, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Cotton Plant",
        id: 40000002,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10019600, 10019800, 10005602, 10019601, 10019601, 10024001, 0,
        ],
    },
    Monster {
        name: "Flower",
        id: 40000003,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005300, 0, /*null*/
            10005603, 10005400, 10005400, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fire Flower",
        id: 40000004,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 100,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025205, 0, /*null*/
            10005604, 10043201, 10043201, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Ice Flower",
        id: 40000005,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025205, 0, /*null*/
            10005605, 10043203, 10043203, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tornado Flower",
        id: 40000006,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 100,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025205, 0, /*null*/
            10005606, 10043207, 10043207, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Earth Flower",
        id: 40000007,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10025205, 0, /*null*/
            10005607, 10043208, 10043208, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Light Flower",
        id: 40000008,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 100,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10004000, 0, /*null*/
            10005608, 10043204, 10043204, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Dark Flower",
        id: 40000009,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            0, /*null*/
            10053200, 0, /*null*/
            10005609, 10043209, 10043209, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wood Tree",
        id: 40000010,
        monster_type: "CULTURE_TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10016200, 10005610, 10028400, 10028400, 10028400, 10016300, 10016401, 0,
        ],
    },
    Monster {
        name: "Eastern Fruit Tree",
        id: 40000011,
        monster_type: "CULTURE_TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10002800, 10003650, 10005612, 10002900, 10002900, 10002801, 0,
        ],
    },
    Monster {
        name: "Western Fruie Tree",
        id: 40000012,
        monster_type: "CULTURE_TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003000, 10003650, 10005613, 10003100, 10003100, 10005611, 0,
        ],
    },
    Monster {
        name: "Southern Fruit Tree",
        id: 40000013,
        monster_type: "CULTURE_TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003300, 10003650, 10005614, 10003201, 10003201, 10005611, 0,
        ],
    },
    Monster {
        name: "Northern Fruit Tree",
        id: 40000014,
        monster_type: "CULTURE_TREE_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10003400, 10003650, 10005615, 10003500, 10003500, 10005611, 0,
        ],
    },
    Monster {
        name: "Mushroom",
        id: 40000015,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006900, 10007000, 10005616, 10006999, 10006999, 10007100, 0,
        ],
    },
    Monster {
        name: "Herb",
        id: 40000016,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10004901, 10003601, 10005617, 10004903, 10004903, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Herb",
        id: 40000017,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10004906, 10003601, 10005618, 10004907, 10004907, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Herb",
        id: 40000018,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10004905, 10003601, 10005619, 10004908, 10004908, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Herb",
        id: 40000019,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10004902, 10003601, 10005620, 10004904, 10004904, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley",
        id: 40000020,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10005900, 10007900, 10005621, 10007900, 10007900, 0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley Weed",
        id: 40000021,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley Weed (Large)",
        id: 40000022,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 3.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Barley Weed (Medium)",
        id: 40000023,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Weed (Large)",
        id: 40000024,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Red Flower",
        id: 40000025,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blue Flower",
        id: 40000026,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Yellow Flower",
        id: 40000027,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Orange Flower",
        id: 40000028,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "White Flower",
        id: 40000029,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Purple Flower",
        id: 40000030,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Flower",
        id: 40000031,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mushroom(Small)",
        id: 40000032,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blue Mushroom(Small)",
        id: 40000033,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Green Mushroom(Small)",
        id: 40000034,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Orange Mushroom(Small)",
        id: 40000035,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Brown Mushroom(Small)",
        id: 40000036,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Black Mushroom(Small)",
        id: 40000037,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 0.7,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Mushroom(Large)",
        id: 40000038,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Blue Mushroom(Large)",
        id: 40000039,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Green Mushroom(Large)",
        id: 40000040,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Orange Mushroom(Large)",
        id: 40000041,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Brown Mushroom(Large)",
        id: 40000042,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Black Mushroom(Large)",
        id: 40000043,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Sakura",
        id: 40000044,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 100,
        sp: 1,
        mp: 1,
        def: [65, 0],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nose Peanut",
        id: 40000045,
        monster_type: "CULTURE_PLANT_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 0],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Garnet",
        id: 40050001,
        monster_type: "KNIGHTS_WAR_INFO_MATERIAL",
        level: 0,
        scale: 0.5,
        exp: 0,
        job: 0,
        hp: 2,
        sp: 1,
        mp: 1,
        def: [99, 5],
        mdef: [99, 5],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011400, 10011400, 0,
        ],
    },
    Monster {
        name: "Aquamarine",
        id: 40050003,
        monster_type: "KNIGHTS_WAR_INFO_MATERIAL",
        level: 0,
        scale: 0.8,
        exp: 0,
        job: 0,
        hp: 5,
        sp: 1,
        mp: 1,
        def: [99, 250],
        mdef: [99, 250],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011400, 10011400, 0,
        ],
    },
    Monster {
        name: "Emerald",
        id: 40050005,
        monster_type: "KNIGHTS_WAR_INFO_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 10,
        sp: 1,
        mp: 1,
        def: [99, 600],
        mdef: [99, 600],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011400, 10011400, 0,
        ],
    },
    Monster {
        name: "Tourmaline",
        id: 40050050,
        monster_type: "KNIGHTS_WAR_INFO_MATERIAL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 10000,
        sp: 1,
        mp: 1,
        def: [0, 1],
        mdef: [0, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            10011400, 10011400, 0,
        ],
    },
    Monster {
        name: "East Emblem",
        id: 40051000,
        monster_type: "KNIGHTS_WAR_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 80000,
        sp: 1,
        mp: 1,
        def: [30, 0],
        mdef: [30, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "West Emblem",
        id: 40052000,
        monster_type: "KNIGHTS_WAR_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 80000,
        sp: 1,
        mp: 1,
        def: [30, 0],
        mdef: [30, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "South Emblem",
        id: 40053000,
        monster_type: "KNIGHTS_WAR_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 80000,
        sp: 1,
        mp: 1,
        def: [30, 0],
        mdef: [30, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "North Emblem",
        id: 40054000,
        monster_type: "KNIGHTS_WAR_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 80000,
        sp: 1,
        mp: 1,
        def: [30, 0],
        mdef: [30, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Treasure Chest",
        id: 40060001,
        monster_type: "TREASURE_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 3,
        sp: 10,
        mp: 10,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000900, 10000901, 10000903, 10000904, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Container",
        id: 40060002,
        monster_type: "CONTAINER_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 3,
        sp: 10,
        mp: 10,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000900, 10000902, 10000903, 10000905, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Wooden Box",
        id: 40060003,
        monster_type: "TIMBER_BOX_MATERIAL",
        level: 0,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 3,
        sp: 10,
        mp: 10,
        def: [99, 10],
        mdef: [99, 10],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000901, 10000902, 10000904, 10000905, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Flower Monument",
        id: 40061001,
        monster_type: "COLISEUM_MATERIAL",
        level: 0,
        scale: 1.25,
        exp: 0,
        job: 0,
        hp: 1500,
        sp: 10,
        mp: 10,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Flower Monument",
        id: 40061002,
        monster_type: "COLISEUM_MATERIAL",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 2000,
        sp: 10,
        mp: 10,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Flower Monument",
        id: 40061003,
        monster_type: "COLISEUM_MATERIAL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 3500,
        sp: 10,
        mp: 10,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Rock Monument",
        id: 40061004,
        monster_type: "COLISEUM_MATERIAL",
        level: 0,
        scale: 1.25,
        exp: 0,
        job: 0,
        hp: 1500,
        sp: 10,
        mp: 10,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Rock Monument",
        id: 40061005,
        monster_type: "COLISEUM_MATERIAL",
        level: 0,
        scale: 1.5,
        exp: 0,
        job: 0,
        hp: 2000,
        sp: 10,
        mp: 10,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Rock Monument",
        id: 40061006,
        monster_type: "COLISEUM_MATERIAL",
        level: 0,
        scale: 2.5,
        exp: 0,
        job: 0,
        hp: 3500,
        sp: 10,
        mp: 10,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Healer Balloon Rabbit",
        id: 40610000,
        monster_type: "HUMAN",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Fluffy Bunny",
        id: 40610001,
        monster_type: "HUMAN_NOTOUCH",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Tiny Cheerleader Group",
        id: 40620000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 6,
        hp: 23,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10000104, 10032800, 10001805, 10032803, 0, /*null*/
            10034507, 10034508, 0,
        ],
    },
    Monster {
        name: "Healing Jumbo Parfait",
        id: 40630000,
        monster_type: "MAGIC_CREATURE",
        level: 1,
        scale: 1.0,
        exp: 15,
        job: 0,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Muffler Lizard (Pink)",
        id: 14270003,
        monster_type: "ANIMAL_BOSS",
        level: 1,
        scale: 1.0,
        exp: 4500,
        job: 4500,
        hp: 1,
        sp: 999,
        mp: 999,
        def: [1, 1],
        mdef: [1, 1],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "RockBird (Red)",
        id: 30150014,
        monster_type: "BIRD",
        level: 23,
        scale: 0.7,
        exp: 1284,
        job: 744,
        hp: 150,
        sp: 50,
        mp: 50,
        def: [0, 0],
        mdef: [0, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            10007800, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Wheat",
        id: 30150015,
        monster_type: "PLANT_MATERIAL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [35, 1],
        mdef: [35, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            21050125, 10007500, 0, /*null*/
            10034507, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Nymph Tree",
        id: 30150016,
        monster_type: "TREE_MATERIAL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 20,
        sp: 1,
        mp: 1,
        def: [65, 1],
        mdef: [65, 0],
        properties: Properties {
            fire: 0,
            water: 100,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            21050125, 10007500, 0, /*null*/
            10034507, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Giant Ice Rock",
        id: 30150017,
        monster_type: "ROCK_MATERIAL",
        level: 0,
        scale: 2.0,
        exp: 0,
        job: 0,
        hp: 60,
        sp: 1,
        mp: 1,
        def: [70, 1],
        mdef: [70, 0],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            21050125, 10007500, 0, /*null*/
            10034507, 0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Brawler-DEM",
        id: 30150018,
        monster_type: "HUMAN",
        level: 0,
        scale: 1.0,
        exp: 204,
        job: 204,
        hp: 150,
        sp: 100,
        mp: 150,
        def: [12, 20],
        mdef: [10, 15],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Super Ruler Of Titania",
        id: 99000000,
        monster_type: "ANIMAL_SPBOSS_SKILL",
        level: 99,
        scale: 1.5,
        exp: 20250000,
        job: 18000000,
        hp: 1700000,
        sp: 0,
        mp: 0,
        def: [55, 500],
        mdef: [65, 800],
        properties: Properties {
            fire: 10,
            water: 10,
            wind: 10,
            earth: 10,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            21050199, 21050199, 21050199, 21050199, 21050199, 21050199, 21050199, 0,
        ],
    },
    Monster {
        name: "Super Dominion Dragon",
        id: 99000001,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 99,
        scale: 2.0,
        exp: 20250000,
        job: 18000000,
        hp: 5000000,
        sp: 0,
        mp: 0,
        def: [60, 850],
        mdef: [55, 800],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 20,
        },
        drop_ids: [
            21050200, 21050200, 21050200, 21050200, 21050200, 21050200, 21050200, 0,
        ],
    },
    Monster {
        name: "Super Dominion",
        id: 99000002,
        monster_type: "HUMAN_BOSS",
        level: 85,
        scale: 1.5,
        exp: 6750000,
        job: 5400000,
        hp: 2000000,
        sp: 100,
        mp: 100,
        def: [25, 300],
        mdef: [65, 300],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 70,
            dark: 0,
        },
        drop_ids: [
            21050201, 21050201, 21050201, 21050201, 21050201, 21050201, 21050201, 0,
        ],
    },
    Monster {
        name: "DEM-DH",
        id: 99000003,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 99,
        scale: 1.0,
        exp: 20250000,
        job: 18000000,
        hp: 1600000,
        sp: 1000,
        mp: 1000,
        def: [75, 800],
        mdef: [75, 800],
        properties: Properties {
            fire: 50,
            water: 50,
            wind: 100,
            earth: 50,
            light: 0,
            dark: 200,
        },
        drop_ids: [
            21050202, 21050202, 21050202, 21050202, 21050202, 21050202, 21050202, 0,
        ],
    },
    Monster {
        name: "Super Hell King",
        id: 99000004,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 95,
        scale: 4.5,
        exp: 9000000,
        job: 6750000,
        hp: 2400000,
        sp: 500,
        mp: 500,
        def: [50, 500],
        mdef: [50, 500],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 80,
        },
        drop_ids: [
            21050203, 21050203, 21050203, 21050203, 21050203, 21050203, 21050203, 0,
        ],
    },
    Monster {
        name: "Super Thunder King",
        id: 99000005,
        monster_type: "BIRD_SPBOSS_SKILL",
        level: 95,
        scale: 4.5,
        exp: 9000000,
        job: 6750000,
        hp: 800000,
        sp: 500,
        mp: 500,
        def: [50, 500],
        mdef: [50, 500],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 50,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            21050204, 21050204, 21050204, 21050204, 21050204, 21050204, 21050204, 0,
        ],
    },
    Monster {
        name: "Super Bird",
        id: 99000006,
        monster_type: "BIRD_BOSS",
        level: 85,
        scale: 2.0,
        exp: 6750000,
        job: 5400000,
        hp: 2000000,
        sp: 300,
        mp: 300,
        def: [30, 300],
        mdef: [85, 300],
        properties: Properties {
            fire: 50,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            21050205, 21050205, 21050205, 21050205, 21050205, 21050205, 21050205, 0,
        ],
    },
    Monster {
        name: "DEM?|00F2",
        id: 99000007,
        monster_type: "HUMAN",
        level: 60,
        scale: 1.0,
        exp: 103950,
        job: 44550,
        hp: 64000,
        sp: 150,
        mp: 150,
        def: [45, 220],
        mdef: [20, 150],
        properties: Properties {
            fire: 8,
            water: 0,
            wind: 8,
            earth: 0,
            light: 0,
            dark: 14,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "DEM-NS-5 SONNY",
        id: 99000008,
        monster_type: "HUMAN",
        level: 80,
        scale: 1.0,
        exp: 18000,
        job: 20250,
        hp: 60000,
        sp: 150,
        mp: 150,
        def: [75, 500],
        mdef: [55, 500],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Hot Wasabi",
        id: 99999996,
        monster_type: "PLANT_BOSS_SKILL",
        level: 80,
        scale: 5.0,
        exp: 259200,
        job: 189000,
        hp: 1900000,
        sp: 800,
        mp: 800,
        def: [99, 700],
        mdef: [70, 700],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 100,
            earth: 0,
            light: 100,
            dark: 0,
        },
        drop_ids: [
            10009500, 10050300, 10012100, 10023502, 60023180, 10012100, 10049500, 0,
        ],
    },
    Monster {
        name: "Red Alert",
        id: 99999997,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 70,
        scale: 1.0,
        exp: 18225,
        job: 16200,
        hp: 30000,
        sp: 500,
        mp: 500,
        def: [70, 600],
        mdef: [70, 700],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 20,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            10006300, 10006400, 10020300, 10006650, 10020210, 10006650, 10050920,
        ],
    },
    Monster {
        name: "Golden Hornet",
        id: 99999998,
        monster_type: "INSECT_BOSS_SKILL",
        level: 80,
        scale: 5.0,
        exp: 259200,
        job: 189000,
        hp: 777,
        sp: 800,
        mp: 800,
        def: [99, 700],
        mdef: [70, 700],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 100,
            earth: 0,
            light: 100,
            dark: 0,
        },
        drop_ids: [
            10009500, 10050300, 10012100, 10023502, 60023180, 10012100, 10049500, 0,
        ],
    },
    Monster {
        name: "Ferion",
        id: 16760000,
        monster_type: "HUMAN",
        level: 113,
        scale: 0.85,
        exp: 20250000,
        job: 18000000,
        hp: 6500000,
        sp: 1000,
        mp: 1000,
        def: [75, 650],
        mdef: [75, 800],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Homunculus Ferion",
        id: 16760001,
        monster_type: "HUMAN",
        level: 130,
        scale: 0.85,
        exp: 20250000,
        job: 18000000,
        hp: 10000000,
        sp: 0,
        mp: 0,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Demon",
        id: 16760002,
        monster_type: "HUMAN_BOSS",
        level: 113,
        scale: 1.0,
        exp: 954000,
        job: 517500,
        hp: 4500000,
        sp: 0,
        mp: 0,
        def: [70, 650],
        mdef: [70, 750],
        properties: Properties {
            fire: 20,
            water: 0,
            wind: 0,
            earth: 30,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0,
        ],
    },
    Monster {
        name: "Lady Frost",
        id: 16760003,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 0.85,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10128850, 99901002, 10128850, 99901002, 0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Mother",
        id: 99900000,
        monster_type: "HUMAN",
        level: 113,
        scale: 1.0,
        exp: 0,
        job: 954000,
        hp: 4500000,
        sp: 0,
        mp: 0,
        def: [70, 650],
        mdef: [70, 750],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Baby Emil Dragon",
        id: 99900001,
        monster_type: "ANIMAL_BOSS_SKILL_NOTPTDROPRANGE",
        level: 113,
        scale: 0.45,
        exp: 0,
        job: 954000,
        hp: 4500000,
        sp: 0,
        mp: 0,
        def: [70, 650],
        mdef: [70, 750],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Baby Dominion Dragon",
        id: 99900002,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 150,
        scale: 0.3,
        exp: 0,
        job: 405000,
        hp: 1000000,
        sp: 0,
        mp: 0,
        def: [60, 430],
        mdef: [55, 550],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            24, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Baby DEM-Dragon",
        id: 99900003,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 150,
        scale: 0.3,
        exp: 0,
        job: 405000,
        hp: 1000000,
        sp: 0,
        mp: 0,
        def: [60, 430],
        mdef: [55, 550],
        properties: Properties {
            fire: 0,
            water: 20,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            24, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Baby Titania Dragon",
        id: 99900004,
        monster_type: "ANIMAL_BOSS_SKILL",
        level: 118,
        scale: 0.3,
        exp: 0,
        job: 360000,
        hp: 958000,
        sp: 0,
        mp: 0,
        def: [60, 400],
        mdef: [60, 500],
        properties: Properties {
            fire: 0,
            water: 30,
            wind: 10,
            earth: 10,
            light: 30,
            dark: 0,
        },
        drop_ids: [
            24, 10006402, 10020404, 10009600, 10050300, 0, /*null*/
            10037700, 10049500,
        ],
    },
    Monster {
        name: "Dark Feather",
        id: 99900005,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Bawoo",
        id: 99900006,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Shabotan",
        id: 99900007,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Loki",
        id: 99900008,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Balulu",
        id: 99900009,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Seahorse",
        id: 99900010,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Pepen",
        id: 99900011,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Salamander",
        id: 99900012,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Minnie-Do",
        id: 99900013,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "White Familiar",
        id: 99900014,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Automedic",
        id: 99900015,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Death",
        id: 99900016,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Angel Feather",
        id: 99900017,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Tiny",
        id: 99900018,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Dumpty",
        id: 99900019,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Momo",
        id: 99900020,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Necro Armor",
        id: 99900021,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Umbrella",
        id: 99900022,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Cocokko",
        id: 99900023,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Tarantula",
        id: 99900024,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Bear",
        id: 99900025,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Flyfish",
        id: 99900026,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Tentacle",
        id: 99900027,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Mummy",
        id: 99900028,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Draki",
        id: 99900029,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Book",
        id: 99900030,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Tiny Zero",
        id: 99900031,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Tiny Zero+",
        id: 99900032,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Pururu",
        id: 99900033,
        monster_type: "HUMAN_BOSS_SKILL",
        level: 130,
        scale: 1.7,
        exp: 0,
        job: 3,
        hp: 10000000,
        sp: 45000,
        mp: 45000,
        def: [75, 700],
        mdef: [80, 750],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Mandra Wasabi",
        id: 27000000,
        monster_type: "PLANT",
        level: 130,
        scale: 1.0,
        exp: 0,
        job: 289500,
        hp: 12750,
        sp: 0,
        mp: 0,
        def: [45, 500],
        mdef: [70, 300],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 50,
            dark: 0,
        },
        drop_ids: [
            3, 10004908, 10004901, 10150801, 10019205, 0, /*null*/
            10009600, 90000045,
        ],
    },
    Monster {
        name: "A. Mud Hand",
        id: 27000001,
        monster_type: "ROCK",
        level: 130,
        scale: 1.0,
        exp: 0,
        job: 321000,
        hp: 14400,
        sp: 0,
        mp: 0,
        def: [40, 470],
        mdef: [50, 240],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 75,
            dark: 0,
        },
        drop_ids: [
            3, 10015400, 10000210, 10150801, 99901009, 0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Deep Toad",
        id: 27000002,
        monster_type: "WATER_ANIMAL",
        level: 132,
        scale: 1.0,
        exp: 0,
        job: 202500,
        hp: 13250,
        sp: 0,
        mp: 0,
        def: [50, 300],
        mdef: [45, 500],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 60,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10032550, 10150801, 10000609, 99901009, 0, /*null*/
            0, /*null*/
            90000045,
        ],
    },
    Monster {
        name: "A. Reckless Gecko",
        id: 27000003,
        monster_type: "ANIMAL",
        level: 136,
        scale: 1.5,
        exp: 0,
        job: 289500,
        hp: 8950,
        sp: 0,
        mp: 0,
        def: [35, 470],
        mdef: [25, 370],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 30,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10001804, 10001900, 10150801, 99901009, 0, /*null*/
            60023200, 0, /*null*/
        ],
    },
    Monster {
        name: "Tainted Arlaune",
        id: 27000004,
        monster_type: "PLANT_BOSS",
        level: 175,
        scale: 3.0,
        exp: 0,
        job: 7071000,
        hp: 320500,
        sp: 0,
        mp: 0,
        def: [75, 500],
        mdef: [85, 800],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 75,
            dark: 0,
        },
        drop_ids: [
            15, 99902000, 10070200, 10070300, 10050350, 10037600, 99902000, 0, /*null*/
        ],
    },
    Monster {
        name: "A. Rotting Pururu",
        id: 27000005,
        monster_type: "UNDEAD",
        level: 135,
        scale: 1.2,
        exp: 0,
        job: 235710,
        hp: 13300,
        sp: 0,
        mp: 0,
        def: [50, 300],
        mdef: [35, 365],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10032800, 10006652, 10150801, 99901009, 10032809, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Catoblepas",
        id: 27000006,
        monster_type: "ANIMAL",
        level: 139,
        scale: 1.0,
        exp: 0,
        job: 253590,
        hp: 18750,
        sp: 0,
        mp: 0,
        def: [40, 420],
        mdef: [50, 575],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10020550, 10013350, 10150801, 99901009, 0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Haniwa Fighter",
        id: 27000007,
        monster_type: "ROCK",
        level: 137,
        scale: 1.0,
        exp: 0,
        job: 293700,
        hp: 21150,
        sp: 0,
        mp: 0,
        def: [60, 650],
        mdef: [45, 400],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 75,
            dark: 0,
        },
        drop_ids: [
            3, 10014650, 10000603, 10150801, 99901009, 50125900, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Shadow Servant",
        id: 27000008,
        monster_type: "HUMAN",
        level: 141,
        scale: 1.0,
        exp: 0,
        job: 372000,
        hp: 17950,
        sp: 0,
        mp: 0,
        def: [45, 300],
        mdef: [65, 650],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10000409, 10020309, 10150801, 99901009, 50082100, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Dungeon Fly",
        id: 27000009,
        monster_type: "INSECT",
        level: 135,
        scale: 1.0,
        exp: 0,
        job: 226800,
        hp: 15400,
        sp: 0,
        mp: 0,
        def: [50, 470],
        mdef: [35, 475],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10018204, 10016800, 10150801, 99901009, 0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Cursed Phoenix",
        id: 27000010,
        monster_type: "BIRD_BOSS",
        level: 177,
        scale: 3.5,
        exp: 0,
        job: 9525000,
        hp: 401500,
        sp: 0,
        mp: 0,
        def: [45, 700],
        mdef: [85, 400],
        properties: Properties {
            fire: 0,
            water: 25,
            wind: 0,
            earth: 25,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            15, 99902001, 10018301, 10021700, 10050350, 10037600, 99902001, 0, /*null*/
        ],
    },
    Monster {
        name: "A. Charm Pururu",
        id: 27000011,
        monster_type: "PLANT_NOTOUCH",
        level: 138,
        scale: 0.4,
        exp: 0,
        job: 36750,
        hp: 5000,
        sp: 0,
        mp: 0,
        def: [25, 200],
        mdef: [25, 200],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 100,
        },
        drop_ids: [
            3, 10032800, 10001911, 10051900, 10150801, 0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Bolt Pururu",
        id: 27000012,
        monster_type: "PLANT",
        level: 140,
        scale: 1.2,
        exp: 0,
        job: 269550,
        hp: 24400,
        sp: 0,
        mp: 0,
        def: [60, 250],
        mdef: [45, 300],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 100,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10032807, 10011207, 10011208, 10150801, 99901009, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Standard Pururu",
        id: 27000013,
        monster_type: "PLANT",
        level: 140,
        scale: 1.0,
        exp: 0,
        job: 381000,
        hp: 21250,
        sp: 0,
        mp: 0,
        def: [45, 500],
        mdef: [35, 500],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10032804, 10032800, 10000104, 10150801, 99901009, 50119300, 0, /*null*/
        ],
    },
    Monster {
        name: "A. Snow Pururu",
        id: 27000014,
        monster_type: "PLANT",
        level: 140,
        scale: 1.0,
        exp: 0,
        job: 435000,
        hp: 18790,
        sp: 0,
        mp: 0,
        def: [45, 450],
        mdef: [65, 450],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 100,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10032811, 10009000, 10000609, 10150801, 99901009, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Ball of Steel",
        id: 27000015,
        monster_type: "PLANT",
        level: 145,
        scale: 2.0,
        exp: 0,
        job: 60,
        hp: 30,
        sp: 0,
        mp: 0,
        def: [96, 300],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10015800, 10015600, 10015710, 10150801, 99901009, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "A. Sweeper",
        id: 27000016,
        monster_type: "PLANT",
        level: 140,
        scale: 3.0,
        exp: 0,
        job: 298950,
        hp: 31250,
        sp: 0,
        mp: 0,
        def: [45, 300],
        mdef: [65, 450],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10032802, 10002202, 10000602, 10150801, 99901009, 60072500, 0, /*null*/
        ],
    },
    Monster {
        name: "Abnormality",
        id: 27000017,
        monster_type: "PLANT_BOSS",
        level: 185,
        scale: 20.0,
        exp: 0,
        job: 9000000,
        hp: 450000,
        sp: 0,
        mp: 0,
        def: [50, 800],
        mdef: [50, 800],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 50,
            earth: 50,
            light: 50,
            dark: 50,
        },
        drop_ids: [
            3, 99902002, 10066500, 50059300, 10050350, 10037600, 99902002, 10136900,
        ],
    },
    Monster {
        name: "Satanic Stone",
        id: 27000018,
        monster_type: "MAGIC_CREATURE_BOSS_SKILL",
        level: 140,
        scale: 1.5,
        exp: 45,
        job: 675,
        hp: 85000,
        sp: 0,
        mp: 0,
        def: [20, 100],
        mdef: [70, 250],
        properties: Properties {
            fire: 0,
            water: 50,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            6, 99901009, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Teralith Titan",
        id: 27000019,
        monster_type: "MAGIC_CREATURE_BOSS",
        level: 200,
        scale: 1.5,
        exp: 0,
        job: 15772500,
        hp: 700000,
        sp: 0,
        mp: 0,
        def: [50, 1200],
        mdef: [50, 2000],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 55,
            dark: 0,
        },
        drop_ids: [
            3, 99902003, 10001151, 10050350, 10017300, 10017700, 60032800, 0, /*null*/
        ],
    },
    Monster {
        name: "Abyssal Flower",
        id: 27000101,
        monster_type: "PLANT_MATERIAL",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 300000,
        hp: 90000,
        sp: 999,
        mp: 0,
        def: [30, 20],
        mdef: [70, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10005300, 10043209, 10150801, 10000609, 99901009, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Abyssal Flower",
        id: 27000102,
        monster_type: "PLANT_MATERIAL",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 15000,
        hp: 70000,
        sp: 999,
        mp: 0,
        def: [30, 30],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10005300, 10043204, 10150801, 10000610, 99901009, 10099000, 0, /*null*/
        ],
    },
    Monster {
        name: "Abyssal Flower",
        id: 27000103,
        monster_type: "PLANT_MATERIAL",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 300000,
        hp: 80000,
        sp: 999,
        mp: 0,
        def: [30, 30],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10005300, 10043203, 10150801, 10000603, 99901009, 10099000, 0, /*null*/
        ],
    },
    Monster {
        name: "Abyssal Flower",
        id: 27000104,
        monster_type: "PLANT_MATERIAL",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 300000,
        hp: 45000,
        sp: 999,
        mp: 0,
        def: [30, 50],
        mdef: [70, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10005300, 10043209, 10150801, 10000609, 99901009, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Abyssal Flower",
        id: 27000105,
        monster_type: "PLANT_MATERIAL",
        level: 1,
        scale: 1.0,
        exp: 0,
        job: 15000,
        hp: 90000,
        sp: 999,
        mp: 0,
        def: [30, 40],
        mdef: [50, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10005300, 10043204, 10150801, 10000610, 99901009, 10099000, 0, /*null*/
        ],
    },
    Monster {
        name: "Abyssal Tree",
        id: 27000106,
        monster_type: "PLANT_MATERIAL",
        level: 1,
        scale: 1.6,
        exp: 0,
        job: 225000,
        hp: 100000,
        sp: 999,
        mp: 0,
        def: [30, 50],
        mdef: [40, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10016400, 10002807, 10150801, 10000604, 99901009, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Abyssal Tree",
        id: 27000107,
        monster_type: "PLANT_MATERIAL",
        level: 1,
        scale: 1.6,
        exp: 0,
        job: 22500,
        hp: 100000,
        sp: 999,
        mp: 0,
        def: [30, 50],
        mdef: [40, 150],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 10016400, 10002807, 10150801, 10000604, 99901009, 0, /*null*/
            0, /*null*/
        ],
    },
    Monster {
        name: "Concealing Butterfly",
        id: 27001000,
        monster_type: "INSECT",
        level: 100,
        scale: 1.0,
        exp: 0,
        job: 0,
        hp: 50,
        sp: 0,
        mp: 0,
        def: [95, 500],
        mdef: [100, 999],
        properties: Properties {
            fire: 0,
            water: 0,
            wind: 0,
            earth: 0,
            light: 0,
            dark: 0,
        },
        drop_ids: [
            3, 99903500, 0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
            0, /*null*/
        ],
    },
];

pub fn monster_by_id(id: u32) -> Option<&'static Monster> {
    for m in &MONSTERS {
        if m.id == id {
            return Some(m);
        }
    }
    None
}
