// generated file, do not edit

import ArtifactEff_image from "@image/misc/sword"


export default {

    "ArtifactEff": {
        name: "ArtifactEff",
        chs: "Standard ",
        description: "The maximum level of a single upgrade is 1 point",
        badge: ArtifactEff_image,
        config: [
            
            {"default":false,"name":"atk_use","title":"Effective ATK","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"atk_weight","title":"ATK Weight","type":"float"},
            
            {"default":true,"name":"atk_p_use","title":"Effective ATK%","type":"bool"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"atk_p_weight","title":"ATK% Weight","type":"float"},
            
            {"default":false,"name":"hp_use","title":"Effective HP","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"hp_weight","title":"HP Weight","type":"float"},
            
            {"default":false,"name":"hp_p_use","title":"Effective HP%","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"hp_p_weight","title":"HP% Weight","type":"float"},
            
            {"default":false,"name":"def_use","title":"Effective DEF","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"def_weight","title":"DEF Weight","type":"float"},
            
            {"default":false,"name":"def_p_use","title":"Effective DEF%","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"def_p_weight","title":"DEF% Weight","type":"float"},
            
            {"default":true,"name":"critical_use","title":"Effective Crit Rate","type":"bool"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"critical_weight","title":"Crit Rate Weight","type":"float"},
            
            {"default":true,"name":"critical_damage_use","title":"Effective Crit Damage","type":"bool"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"critical_damage_weight","title":"Crit Damage Weight","type":"float"},
            
            {"default":false,"name":"elemental_mastery_use","title":"Effective Elemental Mastery","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"elemental_mastery_weight","title":"Elemental Mastery Weight","type":"float"},
            
            {"default":false,"name":"recharge_use","title":"Effective Energy Recharge","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"recharge_weight","title":"Energy Recharge Weight","type":"float"},
            
        ],
    },

}