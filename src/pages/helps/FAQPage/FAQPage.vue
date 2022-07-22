<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>Help</el-breadcrumb-item>
            <el-breadcrumb-item>FAQ</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row type="flex" justify="center">
            <el-col :xs="24" :sm="18">
                <h1>Target Function FAQ</h1>  
                <h1>1. What is the target function?</h1>
                <p>The target function is used to select a function you want to optimize for. Usually, you would want to use a character’s exclusive, and average damage if that option isn't available. An example could be Hu Tao, she wants HP as it increases her damage a lot, so choosing Hu Tao’s target function would include HP when optimizing. Functions such as Maximum Damage max out your damage without caring about crit rate (Basically for those who love Crit fishing), while other functions such as Individual Properties max out a single stat.</p>
                <h1>2. Are special effects like 4 piece shimenawas or passives taken into account?</h1>
                <p>Some functions such as 4 piece shimenawas are taken into account, in the artifact parameters section, you can set how long the artifact is active for, for example, if using for Ganyu (She has a 10-second elemental skill cooldown) you can drag the slider to 1 as it has full uptime on her. Passives such as Xiangling’s are also taken into account, but only the passives that matter are used. For example, Ganyu’s bow refund passive will not be taken into account.</p>
                <h1>3. Why is there a difference between my in-game damage and the damage given by GAOP?</h1>
                <p>All possible values inside of Genshin Impact are NOT exact values, Genshin Impact does a lot of rounding when calculating damage, and GAOP doesn’t round as often. Due to rounding errors, they may not be the exact number shown in-game, if there is a large difference it may be a bug, if so please report in the GAOP discord server.</p>
            <br>
            <h1>Artifact Potential FAQ</h1>    
            <h1>1. What Does It Do?</h1>
            <p>Artifact rater will give your artifacts a score based on what stats you’re prioritizing or using the built-in features. This can be useful for comparing artifacts without needing to calculate. It is also a more accurate way of measuring how good an artifact is, along with it being MUCH more accurate than other sorts of measurement such as CV (Crit Value, crit rate*2+crit damage).</p>
            <h1>2. Regular Output</h1>
            <p>Regular output is probably what you will use when giving to score to your characters. The default one is 35 crit rate to 100 crit damage, but if your character is using a crit weapon, you can change the stats accordingly. When comparing your artifacts, we are assuming you are using the default 35 crit rate to crit damage. </p>
            <p>Below Average below 400 score</p>
            <p>Average  400-1000 score</p>
            <p>Good 1000-2000 score</p>
            <h1>3. Valid Entry</h1>
            <p>Valid entry is for those who feel the need to be extra and prioritize stats by giving them more “weight”. Weight is a measurement of how much something is worth, in this case, 0 is worth nothing and can go up to whatever you want. For example, if I love flat defence (I Don’t and for legal reasons, this is a joke) I can make crit rate worth 0, crit damage worth 1, and flat defence worth 2, it will value flat defence over crit damage over crit rate.</p>
            <h1>4. Hu Tao/Noelle</h1>
            <p>These options are just to give scores for HP/DEF scaling characters such as HP and Defence. Nothing much for these, but just gives more value for HP subs on pieces.</p>
            </el-col>
        </el-row>
    </div>
</template>

<script>
export default {
    name: "FAQPage",
}
</script>