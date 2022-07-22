<template>
    <el-radio-group
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-radio-button label="NormalAttack">Normal Attack</el-radio-button>
        <el-radio-button label="ChargedAttack">Charged Attack</el-radio-button>
        <el-radio-button label="ElementalSkill">Elemental Skill</el-radio-button>
        <el-radio-button label="ElementalBurst">Elemental Burst</el-radio-button>
    </el-radio-group>
</template>

<script>
export default {
    name: "SelectSkillType",
    props: {
        value: {
            type: String,
            required: true,
        }
    }
}
</script>