import { artifactTags } from "@const/artifact";
import { artifactsData } from "@asset/artifacts";

/**
 * attackPercentage, 0.2 => "攻击力+20%"
 * attackStatic, 20 => "攻击力+20"
 */
export function displayedTag(name, value) {
    let tagData = artifactTags[name];
    if (!tagData) {
        throw "tag name not exist";
    }

    let left = "";
    switch (name) {
        case "attackPercentage":
        case "attackStatic":
            left = "ATK";
            break;
        case "lifePercentage":
        case "lifeStatic":
            left = "HP";
            break;
        case "defendPercentage":
        case "defendStatic":
            left = "DEF";
            break;
        default:
            left = tagData.chs;
            break;
    }

    if (tagData.percentage) {
        let s = (value * 100).toFixed(1);
        return left + "+" + s + "%";
    } else {
        return left + "+" + value;
    }
}

/**
 * attackPercentage, 0.1 -> "10"
 * attackStatic, 10 -> "10"
 */
export function convertDisplayTagValue(name, value) {
    let tagData = artifactTags[name];
    if (!tagData) {
        throw "tag name not exist";
    }

    if (tagData.percentage) {
        return (value * 100).toFixed(1);
    }
    return value;
}

export function getArtifactThumbnailURL(setName) {
    let artData = artifactsData[setName];

    if (artData["flower"]) {
        return artData["flower"].url;
    }

    return artData["head"].url;
}

export function getArtifactRealValue(name, value) {
    let temp = parseFloat(value);
    if (artifactTags[name].percentage) {
        return temp / 100;
    }
    return temp;
}

export function getDetailName(setName, position) {
    return artifactsData[setName][position].chs;
}

export function getArtifactUpCount(artifact) {
    // historical reasons, star field may not exist, default to 5
    let star = artifact.star ?? 5;
    if (star > 5 || star < 1) {
        return 0;
    }

    let level = 20;
    if (Object.prototype.hasOwnProperty.call(artifact, "level")) {
        level = artifact.level;
    }
    level = Math.min(level, 20);
    level = Math.max(level, 0);

    return star - Math.floor(level / 4);
}

export function positionToLocale(name) {
    switch (name) {
        case "flower":
            return "Flower of Life";
        case "feather":
            return "Plume of Death";
        case "sand":
            return "Sands of Eon";
        case "cup":
            return "Goblet";
        case "head":
            return "Circlet";
    }
}

export function capitalize(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}