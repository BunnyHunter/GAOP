<template>
    <el-table
        :data="tableDataForElementUI"
        size="small"

    >
        <el-table-column
            prop="chs"
            label="Damage Type"
        ></el-table-column>
        <el-table-column
            label="Damage"
        >
            <template v-slot="{ row }">
                <span v-if="row.chs === 'Electro Charge'"
                    style="color: #c250ff"
                >{{ row.value.toFixed(1) }}</span>
                <span v-else-if="row.chs === 'Overload'"
                      style="color: #ff335a"
                >{{ row.value.toFixed(1) }}</span>
                <span v-else>{{ row.value.toFixed(1) }}</span>
            </template>
        </el-table-column>
    </el-table>
</template>

<script>
export default {
    name: "TransformativeDamage",
    props: ["data"],
    computed: {
        tableDataForElementUI() {
            let results = []
            results.push({ chs: "Electro Charge", value: this.data.electro_charged })
            results.push({ chs: "Overload", value: this.data.overload })
            results.push({ chs: "Shatter", value: this.data.shatter })
            results.push({ chs: "SuperConduct", value: this.data.superconduct })
            results.push({ chs: "Swirl（Electro）", value: this.data.swirl_electro })
            results.push({ chs: "Swirl（Pyro）", value: this.data.swirl_pyro })
            results.push({ chs: "Swirl（Cryo）", value: this.data.swirl_cryo })
            results.push({ chs: "Swirl（Hydro）", value: this.data.swirl_hydro })
            return results
        }
    }
}
</script>

<style lang="scss" scoped>


</style>