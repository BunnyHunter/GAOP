<div align="center">


# GAOP Genshin Artifact Optimizer 



</div>

## Introduction

-Artifacts are automatically calculated

-Team Artifacts are automatically calculated

-Responsive damage calculator

-Artifacts Potential

-Recommendations for Artifacts Upgrades


## run locally
This project depends on [Rust](https://www.rust-lang.org/)
1. Install the Rust toolchain, see the official website for details
2. Install the Rust Webassembly toolchain (wasm-pack)
3. Install node dependencies
````
npm install
````
4. Compile Rust dependencies
````
cd mona
cargo run --bin gen_meta
wasm-pack build
````
5. Run locally
````
npm run serve
````

## Docker
```
docker build -t mona .
docker run -dp 8080:80 mona
```


## contribute
### Add objective function
The target functions are in `mona/src/target_functions/target_functions`
1. Create a new target function file in the corresponding location of the above folder
2. Create a new target function name in `target_functions/target_function_name.rs`
3. Create a struct, which must end with `TargetFunction`
```rust
pub struct NewTargetFunction {
    ...
}
````
4. If the function is set, create a new enum with the same name in `target_functions/target_function_config.rs`
5. Implement two traits for `NewTargetFunction`, `TargetFunctionMetaTrait` and `TargetFunction`
```rust
impl TargetFunctionMetaTrait for NewTargetFunction {
    //metadata for the target function
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::GanyuDefault,
        chs: "chs",
        description: "description",
        tags: "tag1,tag2",
four: TargetFunctionFor::SomeWho(CharacterName::Ganyu),
        image: TargetFunctionMetaImage::Avatar
    };

    //The setting of the target function, no setting can be omitted
    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "melt_rate",
            title: "Melting percentage",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        //create boxed target function
    }
}

impl TargetFunction for NewTargetFunction {
    //can refer to other files
}
```
6. in  `target_functions/target_functions/<element>/mod.rs`In, re-export`NewTargetFunction`
```rust
//in <element>.rs
pub use new_target_function::NewTargetFunction;
```# GAOP
