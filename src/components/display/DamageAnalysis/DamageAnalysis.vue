<template>
    <div>
        <div style="margin-bottom: 16px;">
            <el-radio-group v-model="damageType" size="small" style="margin-right: 24px;">
                <el-radio-button label="normal">{{ normalDamageName }}</el-radio-button>
                <el-radio-button v-if="showMeltOption" label="melt">Melt</el-radio-button>
                <el-radio-button v-if="showVaporizeOption" label="vaporize">Vaporize</el-radio-button>
            </el-radio-group>

            <span class="damage-display" v-if="damageType === 'normal'">{{ Math.round(damageNormal) }}</span>
            <span class="damage-display" v-if="damageType === 'melt'">{{ Math.round(damageMelt) }}</span>
            <span class="damage-display" v-if="damageType === 'vaporize'">{{ Math.round(damageVaporize) }}</span>
        </div>

        <div class="header-row" style="overflow: auto; margin-bottom: 16px;">
            <div>
                <div class="big-title base-damage-region">{{ baseRegionName }}</div>
                <div class="header-row">
                    <damage-analysis-util
                        v-if="atkRatioState.length > 0"
                        :arr="atkState"
                        title="ATK "
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="atkRatioState.length > 0"
                        :arr="atkRatioState"
                        title="ATK Ratio "
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="defRatioState.length > 0"
                        :arr="defState"
                        title="DEF "
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="defRatioState.length > 0"
                        :arr="defRatioState"
                        title="DEF Ratio "
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="hpRatioState.length > 0"
                        :arr="hpState"
                        title="HP "
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="hpRatioState.length > 0"
                        :arr="hpRatioState"
                        title="HP Ratio "
                    ></damage-analysis-util>
                    <damage-analysis-util
                        v-if="extraDamageState.length > 0"
                        :arr="extraDamageState"
                        title="Other "
                    ></damage-analysis-util>
                </div>
            </div>
            <div v-show="!isHeal">
                <div class="big-title critical-region">Crit</div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="criticalState"
                        title="Crit Chance "
                    ></damage-analysis-util>
                    <damage-analysis-util
                        :arr="criticalDamageState"
                        title="Crit Damage "
                    ></damage-analysis-util>
                </div>
            </div>
            <div>
                <div class="big-title bonus-region">Additional</div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="bonusState"
                        :title="bonusRegionName"
                    ></damage-analysis-util>
                </div>
            </div>
            <div v-show="damageType === 'melt' || damageType === 'vaporize'">
                <div class="big-title reaction-ratio-region">Reaction Ratio</div>
                <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center">
                    <span>{{ reactionRatio }}</span>
                </div>
            </div>
            <div v-if="damageType === 'melt'">
                <div class="big-title melt-region">Damage Bonus</div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="meltEnhanceState"
                        title="Melt Damage Bonus "
                    ></damage-analysis-util>
                </div>
            </div>
            <div v-if="damageType === 'vaporize'">
                <div class="big-title vaporize-region">Damage Bonus</div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="vaporizeEnhanceState"
                        title="Vaporize Damage Bonus "
                    ></damage-analysis-util>
                </div>
            </div>
        </div>

        <div v-if="!isHeal" class="header-row">
            <div>
                <div class="big-title def-minus">Defense Reduction </div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="defMinusState"
                        title="Defense Reduction "
                    ></damage-analysis-util>
                    <div></div>
                    <damage-analysis-util
                        :arr="defPenetrationState"
                        title="Defense Penetration "
                    ></damage-analysis-util>
                </div>
            </div>
            <br>
            <div>
                <div class="big-title res-minus">Resistance</div>
                <div class="header-row">
                    <damage-analysis-util
                        :arr="resMinusState"
                        title="Resistance Reduction "
                    ></damage-analysis-util>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import DamageAnalysisUtil from "./DamageAnalysisUtil"

function sum(arr) {
    let s = 0
    for (let item of arr) {
        if (item.checked) {
            s += parseFloat(item.value)
        }
    }
    return s
}

export default {
    name: "DamageAnalysis",
    components: {
        DamageAnalysisUtil
    },
    props: ["enemyConfig", "characterLevel"],
    data() {
        return {
            damageType: "normal",
            element: "Pyro",
            isHeal: false,

            atkState: [{ name: "test", value: 1000, checked: true }],
            atkRatioState: [{ name: "test", value: 1000, checked: true }],
            defState: [],
            defRatioState: [],
            hpState: [],
            hpRatioState: [],
            extraDamageState: [],
            criticalState: [],
            criticalDamageState: [],
            meltEnhanceState: [],
            vaporizeEnhanceState: [],
            defMinusState: [],
            defPenetrationState: [],
            resMinusState: [],
            bonusState: [],
            healingBonusState: []
        }
    },
    methods: {
        setValue(analysis) {
            console.log(analysis)
            let map = {
                "atkState": "atk",
                "atkRatioState": "atk_ratio",
                "defState": "def",
                "defRatioState": "def_ratio",
                "hpState": "hp",
                "hpRatioState": "hp_ratio",
                "extraDamageState": "extra_damage",
                "criticalState": "critical",
                "criticalDamageState": "critical_damage",
                "meltEnhanceState": "melt_enhance",
                "vaporizeEnhanceState": "vaporize_enhance",
                "bonusState": "bonus",
                "defMinusState": "def_minus",
                "defPenetrationState": "def_penetration",
                "resMinusState": "res_minus",
                "healingBonusState": "healing_bonus"
            }
            this.element = analysis.element
            this.isHeal = analysis.is_heal
            this.damageType = "normal"
            for (let key in map) {
                let fromKey = map[key]
                let temp = []
                for (let i in analysis[fromKey]) {
                    temp.push({
                        name: i,
                        checked: true,
                        value: Math.round(analysis[fromKey][i] * 1000) / 1000
                    })
                }
                this[key] = temp
            }
        }
    },
    computed: {
        normalDamageName() {
            const map = {
                "Pyro": "Pyro Damage ",
                "Electro": "Electro Damage ",
                "Hydro": "Hydro Damage ",
                "Anemo": "Anemo Damage ",
                "Geo": "Geo Damage ",
                "Dendro": "Dendro Damage ",
                "Cryo": "Cryo Damage ",
                "Physical": "Physical Damage "
            }
            return map[this.element]
        },

        showMeltOption() {
            return this.element === "Cryo" || this.element === "Pyro"
        },

        showVaporizeOption() {
            return this.element === "Pyro" || this.element === "Hydro"
        },
        
        baseRegionName() {
            if (this.isHeal) {
                return "Base Healing "
            } else {
                return "Base Damage "
            }
        },

        bonusRegionName() {
            if (this.isHeal) {
                return "Healing Bonus "
            } else {
                return "Damage Bonus "
            }
        },

        reactionRatio() {
            let map = {
                "Cryomelt": 1.5,
                "Pyromelt": 2,
                "Pyrovaporize": 1.5,
                "Hydrovaporize": 2
            }

            return map[this.element + this.damageType]
        },

        atk() {
            return sum(this.atkState)
        },

        atkRatio() {
            return sum(this.atkRatioState)
        },

        def() {
            return sum(this.defState)
        },

        defRatio() {
            return sum(this.defRatioState)
        },

        hp() {
            return sum(this.hpState)
        },

        hpRatio() {
            return sum(this.hpRatioState)
        },

        extraDamage() {
            return sum(this.extraDamageState)
        },

        bonus() {
            return sum(this.bonusState)
        },

        healingBonus() {
            return sum(this.healingBonusState)
        },

        critical() {
            return sum(this.criticalState)
        },

        criticalDamage() {
            return sum(this.criticalDamageState)
        },

        meltEnhance() {
            return sum(this.meltEnhanceState)
        },

        vaporizeEnhance() {
            return sum(this.vaporizeEnhanceState)
        },

        defMinus() {
            return sum(this.defMinusState)
        },

        defPenetration() {
            return sum(this.defPenetrationState)
        },

        resMinus() {
            return sum(this.resMinusState)
        },

        baseDamage() {
            return this.atk * this.atkRatio + this.def * this.defRatio + this.hp * this.hpRatio + this.extraDamage;
        },

        damageNormal() {
            const enemyLevel = this.enemyConfig.level
            const characterLevel = this.characterLevel
            const c = 100 + characterLevel
            const def_ratio = c / ((1 - this.defPenetration) * (1 - this.defMinus) * (100 + enemyLevel) + c)
            const res = 0.1 - this.resMinus

            let res_ratio
            if (res > 0.75) {
                res_ratio = 1 / (1 + res * 4)
            } else if (res > 0) {
                res_ratio = 1 - res
            } else {
                res_ratio = 1 - res / 2
            }
            let d
            if (this.isHeal) {
                d = this.baseDamage * (1 + this.healingBonus)
            } else {
                d = this.baseDamage * (1 + this.critical * this.criticalDamage) * (1 + this.bonus) * res_ratio * def_ratio
            }
            return d
        },

        damageMelt() {
            const d = this.damageNormal * this.reactionRatio * (1 + this.meltEnhance)
            return d
        },

        damageVaporize() {
            const d = this.damageNormal * this.reactionRatio * (1 + this.vaporizeEnhance)
            return d
        }
    }
}
</script>

<style scoped lang="scss">
.header-row {
    display: flex;
    // align-items: center;
}

.big-title {
    height: 32px;
    display: flex;
    justify-content: center;
    align-items: center;
    min-width: 100px;

    &.base-damage-region {
        background-color: rgb(34 41 51);
    }

    &.critical-region {
        background-color: rgb(34 41 51);
    }

    &.bonus-region {
        background-color: rgb(34 41 51);
    }

    &.reaction-ratio-region {
        background-color: rgb(34 41 51);
    }

    &.vaporize-region {
        background-color: rgb(34 41 51);
    }

    &.melt-region {
        background-color: rgb(34 41 51);
    }

    &.def-minus {
        background-color: rgb(34 41 51);
    }

    &.res-minus {
        background-color: rgb(34 41 51);
    }
}
</style>