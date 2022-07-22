<template>
    <el-checkbox-group
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-checkbox-button v-if="eleMap.has('fire')" label="fire">Pyro</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('thunder')" label="thunder">Electro</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('water')" label="water">Hydro</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('wind')" label="wind">Anemo</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('rock')" label="rock">Geo</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('ice')" label="ice">Cryo</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('physical')" label="physical">Physical</el-checkbox-button>
    </el-checkbox-group>
</template>

<script>
export default {
    name: "SelectElementMulti",
    props: {
        value: {},
        elements: {
            default: () => {
                return ["fire", "thunder", "water", "wind", "rock", "ice", "physical"];
            }
        },
        multi: { default: false }
    },
    computed: {
        eleMap() {
            let temp = new Set();
            for (let ele of this.elements) {
                temp.add(ele);
            }
            return temp;
        }
    }
}
</script>