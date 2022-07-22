<template>
    <div>
        <el-alert
            title="In fact, these parameters are not that important and can basically be skipped"
            :closable="false"
            type="warning"
            class="hint"
        ></el-alert>
        <div class="config-item">
            <h3 class="config-title">Base Attack</h3>
            <el-alert
                title="Refers to the base attack of character + weapon"
                :closable="false"
                class="hint"
            ></el-alert>
            <el-input v-model="baseAtk"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">Current Attack</h3>
            <!--<el-alert
                title="It can be thought of as removing the attack power of the substat"
                :closable="false"
                class="hint"
            ></el-alert>-->
            <el-input v-model="atk" ></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">Base Crit Rate (Including Ascension Stats)</h3>
            <!-- <el-alert
                title="如果想要更加精确的结果，这里填不穿任何圣遗物时的暴击率"
                :closable="false"
                class="hint"
            ></el-alert> -->
            <el-input v-model="baseCrit" ></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">Base Crit Damage (Including Ascension Stats)</h3>
            <!-- <el-alert
                title="如果想要更加精确的结果，这里填不穿任何圣遗物时的暴击伤害"
                :closable="false"
                class="hint"
            ></el-alert> -->
            <el-input v-model="baseCriticalDamage"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "PotentialFuncConfigDamage",
    data: function () {
        return {
            baseAtk: "700",
            atk: "1500",
            baseCrit: "35",
            baseCriticalDamage: "100",
        }
    },
    methods: {
        compact() {
            return {
                baseAtk: parseFloat(this.baseAtk),
                baseCrit: parseFloat(this.baseCrit)/100,
                baseCriticalDamage: parseFloat(this.baseCriticalDamage)/100,
                atk: parseFloat(this.atk),
            }
        }
    }
}
</script>

<style scoped>
.hint {
    margin-bottom: 16px;
}

</style>