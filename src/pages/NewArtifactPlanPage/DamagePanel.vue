<template>
    <div>
        <el-table
            :data="tableData"
            size="small"
        >
            <el-table-column
                prop="name"
                label="Damage Type"
            ></el-table-column>
            <el-table-column
                prop="expectation"
                label="Average Damage"
            ></el-table-column>
            <el-table-column
                prop="critical"
                label="Crit"
            ></el-table-column>
            <el-table-column
                prop="nonCritical"
                label="Non Crit"
            ></el-table-column>
        </el-table>
    </div>
</template>

<script>
export default {
    name: "DamageList",
    props: {
        analysisFromWasm: {}
    },
    data() {
        return {
            showDialog: false
        }
    },
    methods: {
        // handleClickItem(name) {
        //     this.showDialog = true
        //     const analysis = this.damageList[name]

        //     this.$nextTick(() => {
        //         const component = this.$refs["damageAnalysis"]
        //         component.setValue(analysis)
        //     })
        // }
    },
    computed: {
        element() {
            return this.analysisFromWasm.element
        },

        normalDamageTitle() {
            if (this.analysisFromWasm.is_heal) {
                return "Healing"
            } else {
                const map = {
                    "Pyro": "Pyro Element Damage",
                    "Hydro": "Hydro Element Damage",
                    "Electro": "Electro Element Damage",
                    "Cryo": "Cryo Element Damage",
                    "Dendro": "Dendro Element Damage",
                    "Geo": "Geo Element Damage",
                    "Anemo": "Anemo Element Damage",
                    "Physical": "Physical Damage ",
                }
                return map[this.element]
            }
        },

        tableData() {
            let temp = []
            const NO_DATA = "No Data"

            const r = (x) => Math.round(x)

            temp.push({
                expectation: r(this.analysisFromWasm.normal?.expectation) ?? NO_DATA,
                critical: r(this.analysisFromWasm.normal?.critical) ?? NO_DATA,
                nonCritical: r(this.analysisFromWasm.normal?.non_critical) ?? NO_DATA,
                name: this.normalDamageTitle
            })

            if (this.analysisFromWasm.melt) {
                temp.push({
                    expectation: r(this.analysisFromWasm.melt?.expectation) ?? NO_DATA,
                    critical: r(this.analysisFromWasm.melt?.critical) ?? NO_DATA,
                    nonCritical: r(this.analysisFromWasm.melt?.non_critical) ?? NO_DATA,
                    name: "Melt"
                })
            }
            if (this.analysisFromWasm.vaporize) {
                temp.push({
                    expectation: r(this.analysisFromWasm.vaporize?.expectation) ?? NO_DATA,
                    critical: r(this.analysisFromWasm.vaporize?.critical) ?? NO_DATA,
                    nonCritical: r(this.analysisFromWasm.vaporize?.non_critical) ?? NO_DATA,
                    name: "Vaporize"
                })
            }

            return temp
        }
    }
}
</script>

<style scoped lang="scss">
.item {
    height: 32px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 14px;

    &:hover {
        background-color: rgb(69, 166, 223);
    }

    .name {
        
    }

    .numbers {
        display: flex;
        gap: 4px;
    }

    .number {
        padding: 4px;
        border-radius: 3px;
    }

    .melt {
        color: rgb(63, 63, 63);
        // background-color: rgb(155, 218, 255);
        background-image: url("../../assets/badges/cryo.png");
        // background-size: 48px;
        background-position-x: -20px;
        background-position-y: -30px;
        background-repeat: no-repeat;
    }

    .pyro {
        color: rgb(255, 95, 95);
        background-color: rgb(255, 224, 224);
    }

    .physical {
        color: rgb(71, 71, 71);
        background-color: rgb(218, 218, 218);
    }
}
</style>