# Contribution Objective Function Guidelines

## document
Create the function folder you want to write under `src/assets/target_functions/data`, and create at least two files:
-`xxx.tf.func.js`: the computed function
-`xxx.tf.js`: meta information


A configuration component can be created:
-`xxx.tcfg.vue`

## `xxx.tf.js`
You need to export an Object by default:
````js
import badge from "@asset/badges/sword.png";
import config from "./AboveCrit.tcfg.vue";

export default {
    name: "aboveCrit", //unique identifier
    chs: "fixed crit rate", //Chinese displayed
    description: [ //description
        "Prioritize stacking critical strike rate to a given threshold, then stacking attack and critical damage",
    ],
    tags: [ //tags
        "Damage",
    ],
"for": "common", //If it is exclusive to the role, write the role name, and the public write common
    badge, //icon to display
    config, //configure the vue component of the target function (optional)
}
````

## `xxx.tf.func.js`
You need to export an Object by default:
````js
export default {
    name: "aboveCrit", //same as above
    func: f, //the function to use for the calculation
    needContext: false, //Whether the relic context is required, if you need to use the relic information in the calculation, it should be set to true, but it will reduce the operation efficiency
    version: 2, //fixed
}
````

### Definition of calculation function
E.g:
````js
//attribute: panel
//cArgs: character data
//wArgs: weapon data
//tArgs: target function data
//context: Relic context
//return: a number, the larger the better the fit
function f(attribute, { cArgs, wArgs, tArgs }, context) {
    

    return dps;
}
```

### `cArgs`
```typescript
//角色参数
interface CARGS {
    level: number,
    ascend: boolean,
    skill1: number,
    skill2: number,
    skill3: number,
    constellation: number,
    name: string,
    args: any,
}
```

### `wArgs`
```typescript
//武器参数
interface WARGS {
    name: string,
    level: number,
    ascend: boolean,
    refine: number,
    args: any,
}
```

### `tArgs`
```typescript
//目标函数参数
interface TARGS {
    [key: string]: any
}
```
### `context`
```typescript
interface CONTEXT {
    artifactSet: {
        [key: string]: number, //holy relic name, and its number
    }
}
````

### `attribute`
See [attribute.ts](https://github.com/wormtql/genshin_panel/blob/main/src/attribute/attribute.ts)

## `xxx.tcfg.vue`
Vue components need to implement two methods
````vue
<template>
    ...
</template>

<script>
export default {
    name: "AboveCrit.tcfg",
    data() {
        return {
            threshold: "0.5",
        }
    },
    methods: {
        compact() { //get tArgs
            return {
threshold: parseFloat(this.threshold) ?? 0.5,
            }
        },
        setData(d) { //set tArgs
            this.threshold = d.threshold.toString();
        }
    }
    
}
</script>
````

## other
### Get character skills
You can view the `xxx.skill.js` file under `src/assets/characters/`
````js
import { charactersData } from "@asset/characters";

const skill = charactersData["dadaliya"].skill;
````