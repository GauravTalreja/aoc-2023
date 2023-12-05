use rangemap::RangeMap;

fn main() {
    let mut input = std::io::stdin().lines();

    let seeds = input.next().unwrap().unwrap();
    let seeds = seeds
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap());

    let mut seed_soil = RangeMap::new();
    input
        .by_ref()
        .skip(2)
        .take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
        .for_each(|line| {
            let mut kv = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            let soil = kv.next().unwrap();
            let seed = kv.next().unwrap();
            let range = kv.next().unwrap();
            seed_soil.insert(seed..seed + range, soil);
        });

    let mut soil_fert = RangeMap::new();
    input
        .by_ref()
        .skip(1)
        .take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
        .for_each(|line| {
            let mut kv = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            let fert = kv.next().unwrap();
            let soil = kv.next().unwrap();
            let range = kv.next().unwrap();
            soil_fert.insert(soil..soil + range, fert);
        });

    let mut fert_water = RangeMap::new();
    input
        .by_ref()
        .skip(1)
        .take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
        .for_each(|line| {
            let mut kv = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            let water = kv.next().unwrap();
            let fert = kv.next().unwrap();
            let range = kv.next().unwrap();
            fert_water.insert(fert..fert + range, water);
        });

    let mut water_light = RangeMap::new();
    input
        .by_ref()
        .skip(1)
        .take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
        .for_each(|line| {
            let mut kv = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            let light = kv.next().unwrap();
            let water = kv.next().unwrap();
            let range = kv.next().unwrap();
            water_light.insert(water..water + range, light);
        });

    let mut light_temp = RangeMap::new();
    input
        .by_ref()
        .skip(1)
        .take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
        .for_each(|line| {
            let mut kv = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            let temp = kv.next().unwrap();
            let light = kv.next().unwrap();
            let range = kv.next().unwrap();
            light_temp.insert(light..light + range, temp);
        });

    let mut temp_humid = RangeMap::new();
    input
        .by_ref()
        .skip(1)
        .take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
        .for_each(|line| {
            let mut kv = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            let humid = kv.next().unwrap();
            let temp = kv.next().unwrap();
            let range = kv.next().unwrap();
            temp_humid.insert(temp..temp + range, humid);
        });

    let mut humid_loc = RangeMap::new();
    input
        .by_ref()
        .skip(1)
        .take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
        .for_each(|line| {
            let mut kv = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            let loc = kv.next().unwrap();
            let humid = kv.next().unwrap();
            let range = kv.next().unwrap();
            humid_loc.insert(humid..humid + range, loc);
        });

    //println!("seed_soil:\n{:?}\n\nsoil_fert:\n{:?}\n\nfert_water:\n{:?}\n\nwater_light:\n{:?}\n\nlight_temp:\n{:?}\n\ntemp_humid:\n{:?}\n\nhumid_loc:\n{:?}\n", seed_soil, soil_fert, fert_water, water_light, light_temp, temp_humid, humid_loc);

    let min = seeds
        .map(|seed| {
            let (seed_range, soil) = seed_soil.get_key_value(&seed).unwrap_or((&(0..0), &0));
            let soil = soil + (seed - seed_range.start);
            let (soil_range, fert) = soil_fert.get_key_value(&soil).unwrap_or((&(0..0), &0));
            let fert = fert + (soil - soil_range.start);
            let (fert_range, water) = fert_water.get_key_value(&fert).unwrap_or((&(0..0), &0));
            let water = water + (fert - fert_range.start);
            let (water_range, light) = water_light.get_key_value(&water).unwrap_or((&(0..0), &0));
            let light = light + (water - water_range.start);
            let (light_range, temp) = light_temp.get_key_value(&light).unwrap_or((&(0..0), &0));
            let temp = temp + (light - light_range.start);
            let (temp_range, humid) = temp_humid.get_key_value(&temp).unwrap_or((&(0..0), &0));
            let humid = humid + (temp - temp_range.start);
            let (humid_range, loc) = humid_loc.get_key_value(&humid).unwrap_or((&(0..0), &0));
            let loc = loc + (humid - humid_range.start);
            loc
        })
        .min()
        .unwrap();

    println!("{min}");
}
