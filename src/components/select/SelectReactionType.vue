<template>
    <el-radio-group
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <template v-if="type === 'amp' || type === 'any'">
            <el-radio-button label="vaporize">Vaporize</el-radio-button>
            <el-radio-button label="melt">Melt</el-radio-button>
        </template>
    </el-radio-group>
</template>

<script>
export default {
    name: "SelectReactionType",
    props: ["type",  "value"],

}
</script>