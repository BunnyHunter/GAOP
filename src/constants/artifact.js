export const artifactEff = {
    "4": {
        critical: [0.022, 0.025, 0.028, 0.031],
        lifePercentage: [0.033, 0.037, 0.042, 0.047],
        attackPercentage: [0.033, 0.037, 0.042, 0.047],
        recharge: [0.036, 0.041, 0.047, 0.052],
        defendPercentage: [0.041, 0.047, 0.053, 0.058],
        criticalDamage: [0.044, 0.05, 0.056, 0.062],
        attackStatic: [11, 12, 14, 16],
        defendStatic: [13, 15, 17, 19],
        elementalMastery: [13, 15, 17, 19],
        lifeStatic: [167, 191, 215, 239],
    },
    "5": {
        critical: [0.027, 0.031, 0.035, 0.039],
        lifePercentage: [0.041, 0.047, 0.053, 0.058],
        attackPercentage: [0.041, 0.047, 0.053, 0.058],
        recharge: [0.045, 0.052, 0.058, 0.065],
        defendPercentage: [0.051, 0.058, 0.066, 0.073],
        criticalDamage: [0.054, 0.062, 0.07, 0.078],
        attackStatic: [14, 16, 18, 19],
        defendStatic: [16, 19, 21, 23],
        elementalMastery: [16, 19, 21, 23],
        lifeStatic: [209, 239, 269, 299],
    }
}

export const artifactTags = {
    "cureEffect": {
        "name": "cureEffect",
        "chs": "Healing Bonus ",
        "percentage": true,
        "max": {
            "4": 0.268,
            "5": 0.359
        }
    },
    "criticalDamage": {
        "name": "criticalDamage",
        "chs": "Crit Damage ",
        "percentage": true,
        "max": {
            "4": 0.464,
            "5": 0.622
        }
    },
    "critical": {
        "name": "critical",
        "chs": "Crit Rate ",
        "percentage": true,
        "max": {
            "4": 0.232,
            "5": 0.311
        }
    },
    "attackStatic": {
        "name": "attackStatic",
        "chs": "ATK ",
        "percentage": false,
        "max": {
            "4": 232,
            "5": 311
        }
    },
    "attackPercentage": {
        "name": "attackPercentage",
        "chs": "ATK% ",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "elementalMastery": {
        "name": "elementalMastery",
        "chs": "Elemental Mastery ",
        "percentage": false,
        "max": {
            "4": 139,
            "5": 187
        }
    },
    "recharge": {
        "name": "recharge",
        "chs": "Energy Recharge ",
        "percentage": true,
        "max": {
            "4": 0.387,
            "5": 0.518
        }
    },
    "lifePercentage": {
        "name": "lifePercentage",
        "chs": "HP% ",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "defendPercentage": {
        "name": "defendPercentage",
        "chs": "DEF% ",
        "percentage": true,
        "max": {
            "4": 0.435,
            "5": 0.583
        }
    },
    "lifeStatic": {
        "name": "lifeStatic",
        "chs": "HP ",
        "percentage": false,
        "max": {
            "4": 3571,
            "5": 4780
        }
    },
    "defendStatic": {
        "name": "defendStatic",
        "chs": "DEF ",
        "percentage": false
    },
    "thunderBonus": {
        "name": "thunderBonus",
        "chs": "Electro DMG ",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "fireBonus": {
        "name": "fireBonus",
        "chs": "Pyro DMG ",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "waterBonus": {
        "name": "waterBonus",
        "chs": "Hydro DMG ",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "iceBonus": {
        "name": "iceBonus",
        "chs": "Cryo DMG ",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "windBonus": {
        "name": "windBonus",
        "chs": "Anemo DMG ",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "rockBonus": {
        "name": "rockBonus",
        "chs": "Geo DMG ",
        "percentage": true,
        "max": {
            "4": 0.348,
            "5": 0.466
        }
    },
    "physicalBonus": {
        "name": "physicalBonus",
        "chs": "Physical DMG ",
        "percentage": true,
        "max": {
            "4": 0.435,
            "5": 0.583
        }
    }
}

export const mainStatMap = {
    flower: ["lifeStatic"],
    feather: ["attackStatic"],
    sand: ["attackPercentage", "lifePercentage", "defendPercentage", "elementalMastery", "recharge"],
    cup: [
        "thunderBonus",
        "fireBonus",
        "waterBonus",
        "iceBonus",
        "windBonus",
        "rockBonus",
        "physicalBonus",
        "attackPercentage",
        "lifePercentage",
        "defendPercentage",
        "elementalMastery"
    ],
    head: [
        "critical",
        "criticalDamage",
        "attackPercentage",
        "cureEffect",
        "elementalMastery",
        "lifePercentage",
        "defendPercentage",
    ]
}

export const subStats = [
    "critical",
    "criticalDamage",
    "attackStatic",
    "attackPercentage",
    "lifeStatic",
    "lifePercentage",
    "defendStatic",
    "defendPercentage",
    "elementalMastery",
    "recharge",
]

export const positions = [
    "flower", "feather", "sand", "cup", "head"
]
