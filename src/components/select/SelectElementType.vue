<template>
    <el-radio-group
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-radio-button v-if="eleMap.has('Pyro')" label="Pyro">Pyro</el-radio-button>
        <el-radio-button v-if="eleMap.has('Electro')" label="Electro">Electro</el-radio-button>
        <el-radio-button v-if="eleMap.has('Hydro')" label="Hydro">Hydro</el-radio-button>
        <el-radio-button v-if="eleMap.has('Anemo')" label="Anemo">Anemo</el-radio-button>
        <el-radio-button v-if="eleMap.has('Geo')" label="Geo">Geo</el-radio-button>
        <el-radio-button v-if="eleMap.has('Cryo')" label="Cryo">Cryo</el-radio-button>
        <el-radio-button v-if="eleMap.has('Dendro')" label="Dendro">Dendro</el-radio-button>
        <el-radio-button v-if="eleMap.has('Physical')" label="Physical">Physical</el-radio-button>
    </el-radio-group>
</template>

<script>
export default {
    name: "SelectElementType",
    props: {
        value: {
            type: String,
            required: true,
        },
        elements: {
            default: () => {
                return ["Pyro", "Electro", "Hydro", "Anemo", "Geo", "Cryo", "Dendro", "Physical"];
            }
        },
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