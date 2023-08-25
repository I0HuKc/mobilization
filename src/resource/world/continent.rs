pub mod continent_definition {
    //! Модуль определения континентов

    use noise::{
        Add, Billow, Blend, Cache, Clamp, Constant, Curve, Exponent, Fbm, Max, Min, MultiFractal,
        Multiply, RidgedMulti, ScaleBias, Seedable, Select, Terrace, Turbulence, Worley,
    };

    use crate::resource::world::{
        BADLANDS_LACUNARITY, BADLANDS_TWIST, CONTINENT_FREQUENCY, CONTINENT_LACUNARITY,
        CURRENT_SEED, HILLS_LACUNARITY, HILLS_TWIST, MOUNTAINS_TWIST, MOUNTAIN_GLACIATION,
        MOUNTAIN_LACUNARITY, PLAINS_LACUNARITY, SEA_LEVEL, SHELF_LEVEL, TERRAIN_OFFSET,
    };

    #[allow(dead_code)]
    fn base_continent_definition() {
        /////////////////////////////////////////////////////////////////////////////
        // The Steps Group: IDENTIFYING CONTINENTS
        /////////////////////////////////////////////////////////////////////////////

        /////////////////////////////////////////////////////////////////////////////
        // Subgroup: Basic Continent Definition
        /////////////////////////////////////////////////////////////////////////////
        //
        // In the process of completing the steps, determine the positions and baseline markers of
        // continents in the world.
        //
        // "Base mark" is the mark of the terrain before any terrain
        // objects (mountains, hills, etc.) are placed on that terrain.
        //
        // -1.0 represents the lowest elevation and +1.0 represents the highest elevation.
        /////////////////////////////////////////////////////////////////////////////
        //
        // Step 1: Generate Continents.
        // I put a large number of oclaves in the noise function, so the details will be visible at // high zoom levels.
        // high zoom levels.
        let base_continent_def_fb0 = Fbm::new()
            .set_seed(*CURRENT_SEED)
            .set_frequency(*CONTINENT_FREQUENCY)
            .set_persistence(0.5)
            .set_lacunarity(*CONTINENT_LACUNARITY)
            .set_octaves(14);

        // Step 2: Determining the position of mountain ranges.
        // I use the noise function to change the building obtained in step 1,
        // which allows displaying higher values closer to the sea level.
        let base_continent_def_cu: Curve<[f64; 3]> = Curve::new(&base_continent_def_fb0);
        let base_continent_def_cu: Curve<[f64; 3]> = base_continent_def_cu
            .add_control_point(-2.0000 + *SEA_LEVEL, -1.625 + *SEA_LEVEL)
            .add_control_point(-1.0000 + *SEA_LEVEL, -1.375 + *SEA_LEVEL)
            .add_control_point(0.0000 + *SEA_LEVEL, -0.375 + *SEA_LEVEL)
            .add_control_point(0.0625 + *SEA_LEVEL, 0.125 + *SEA_LEVEL)
            .add_control_point(0.1250 + *SEA_LEVEL, 0.250 + *SEA_LEVEL)
            .add_control_point(0.2500 + *SEA_LEVEL, 1.000 + *SEA_LEVEL)
            .add_control_point(0.5000 + *SEA_LEVEL, 0.250 + *SEA_LEVEL)
            .add_control_point(0.7500 + *SEA_LEVEL, 0.250 + *SEA_LEVEL)
            .add_control_point(1.0000 + *SEA_LEVEL, 0.500 + *SEA_LEVEL)
            .add_control_point(2.0000 + *SEA_LEVEL, 0.500 + *SEA_LEVEL);

        // Step 3. Using the BasicMulti high-frequency module followed by the
        // noise functions to cut fragments out of the mountain ranges,
        // so that the mountain ranges are not completely impassable.
        let base_continent_def_fb1 = Fbm::new()
            .set_seed(*CURRENT_SEED + 1)
            .set_frequency(*CONTINENT_FREQUENCY * 4.34375)
            .set_persistence(0.5)
            .set_lacunarity(*CONTINENT_LACUNARITY)
            .set_octaves(11);

        // Step 4: Scale the value obtained in the previous step (usually close to 1.0).
        let base_continent_def_sb: ScaleBias<[f64; 3]> = ScaleBias::new(&base_continent_def_fb1);
        let base_continent_def_sb: ScaleBias<[f64; 3]> =
            base_continent_def_sb.set_scale(0.375).set_bias(0.625);

        // Step 5: Cutting out continents.
        // From the values obtained in step 2, I cut out chunks with minimum values.
        // This ensures that only the minimum output values from step 3 and step 2
        // contribute to the output values from this function.
        // In most cases, the output value from step 2 will be selected,
        // since the output value of the scaled cutter is usually close to 1.0.
        // Occasionally, the result of step 4 will be smaller than the output value from step 2.
        let base_continent_def_mi: Min<[f64; 3]> =
            Min::new(&base_continent_def_sb, &base_continent_def_cu);

        // Step 6: Bonding the continents.
        // I modify the values obtained in step 1 to ensure,
        // that the output value of this function is between -1,0 and 1,0.
        let base_continent_def_cl: Clamp<[f64; 3]> =
            Clamp::new(&base_continent_def_mi).set_bounds(-1.0, 1.0);

        // Final step of the subgroup.
        // Cache the thread of the previous step.
        let base_continent_def: Cache<Clamp<[f64; 3]>> = Cache::new(base_continent_def_cl);

        /////////////////////////////////////////////////////////////////////////////
        // Subgroup: Continent Definition
        /////////////////////////////////////////////////////////////////////////////
        //
        // While executing the steps, I distort the output value obtained in the
        // result of the `base_continent_definition` function,
        // creating a more realistic landscape.
        //
        // Deforming the base continents creates a more bumpy terrain.
        // with rocks and cracks.
        //
        // -1.0 represents the lowest elevation and +1.0 represents the highest elevation.
        /////////////////////////////////////////////////////////////////////////////

        // Step 1. Using coarse turbulence, I distort the value obtained from the baseline
        // definition of continents by adding some random coarse details to it.
        let continent_def_tu0 = Turbulence::<_>::new(&base_continent_def)
            .set_seed(*CURRENT_SEED + 10)
            .set_frequency(*CONTINENT_FREQUENCY * 15.25)
            .set_power(*CONTINENT_FREQUENCY / 113.75)
            .set_roughness(13);

        // Step 2. Using the intermediate turbulence I distort the values
        // obtained in step 1. I apply higher frequencies but lower power,
        // than in step 1, which allows to add intermediate details.
        let continent_def_tu1 = Turbulence::<_>::new(continent_def_tu0)
            .set_seed(*CURRENT_SEED + 11)
            .set_frequency(*CONTINENT_FREQUENCY * 47.25)
            .set_power(*CONTINENT_FREQUENCY / 433.75)
            .set_roughness(12);

        // Step 3: Deforming the basic definition of continents.
        // I also use turbulence, distorting the result of step 2.
        // Turbulence has a higher frequency, but less power than in step 2.
        // in step 2, which allows to add fine details.
        let continent_def_tu2 = Turbulence::<_>::new(continent_def_tu1)
            .set_seed(*CURRENT_SEED + 12)
            .set_frequency(*CONTINENT_FREQUENCY * 95.25)
            .set_power(*CONTINENT_FREQUENCY / 1019.75)
            .set_roughness(11);

        // Step 4: Selective turbulence.
        // To the entire subset of defined base continents, I apply turbulence,
        // which allows for more turbulent coastlines. The selector function selects
        // output values from the (undeformed) subgroup of base-defined continents and step 3.
        // These are used to output a value from the (undeformed) subset of the base-defined continent definition.
        // The selection boundary is near sea level and has a relatively smooth transition.
        // In fact, only the high regions of the base-defined continents are distorted.
        // The submarine and riparian zones remain unaffected.
        let continent_def_se =
            Select::new(&base_continent_def, &continent_def_tu2, &base_continent_def)
                .set_bounds(*SEA_LEVEL - 0.0375, *SEA_LEVEL + 1000.0375)
                .set_falloff(0.0625);

        // Final step of the subgroup.
        // Caching the obtained result.
        // This is the output value for the whole group `CONTINENT DEFINITION`.
        let continent_def = Cache::new(&continent_def_se);

        /////////////////////////////////////////////////////////////////////////////
        // Group of Steps: DETERMINING THE TYPE OF TERRAIN
        /////////////////////////////////////////////////////////////////////////////

        /////////////////////////////////////////////////////////////////////////////
        // Subgroup: Terrain type definition
        /////////////////////////////////////////////////////////////////////////////
        //
        // As a result of the steps, I determine the position of terrain types in the world.
        // Terrain types are created, in order of increasing roughness,
        // plains, hills, and mountains.
        //
        // The output value of this subgroup is based on the output value of the
        // the output value of the `CONTINENT DEFINITION' group.
        //
        // -1.0 represents the smoothest terrain types (plains and underwater) and
        // +1.0 represents the most rugged terrain types (mountains).
        /////////////////////////////////////////////////////////////////////////////

        // Step 1: Warp the continents.
        // Using turbulence to slightly distort the result of the execution of the
        // of the `CONTINENT DEFINITION' group. This prevents the occurrence of
        // rough terrain exclusively on uplands. Rough terrain areas
        // can appear in the ocean, creating rocky islands and fjords.
        let terrain_type_def_tu = Turbulence::<_>::new(&continent_def)
            .set_seed(*CURRENT_SEED + 20)
            .set_frequency(*CONTINENT_FREQUENCY * 18.125)
            .set_power(*CONTINENT_FREQUENCY / 20.59375 * *TERRAIN_OFFSET)
            .set_roughness(3);

        // Step 2: Shift the roughness probability.
        // I use tracing to sharpen the curved continents at sea level
        // and lowering the slope towards the uplands to narrow the areas //
        // where rugged terrain appears, increasing the "sparseness" of the rugged terrain.
        let terrain_type_def_te = Terrace::new(&terrain_type_def_tu)
            .add_control_point(-1.00)
            .add_control_point(*SHELF_LEVEL + *SEA_LEVEL / 2.0)
            .add_control_point(1.00);

        // Final step of the subgroup.
        // Caching the result obtained.
        // This is the output value for the whole group `LOCATION TYPE DEFINITION`.
        let terrain_type_def = Cache::new(terrain_type_def_te);

        /////////////////////////////////////////////////////////////////////////////
        // The Steps Group: MOUNTAINS
        /////////////////////////////////////////////////////////////////////////////

        /////////////////////////////////////////////////////////////////////////////
        // Subgroup: Defining the base of mountains
        /////////////////////////////////////////////////////////////////////////////
        //
        // As a result of the steps, generate the elevation of the base of the mountain.
        //
        // In other subgroups, I add mountain ranges with decreases to the base elevations.
        //
        // -1.0 represents the smoothest terrain types (plains and underwater) and
        // +1.0 represents the roughest terrain types (mountains).
        /////////////////////////////////////////////////////////////////////////////

        // Step 1: Define a mountain range.
        // I use the ribbed multifractal noise function to generate it.
        let mountain_base_def_rm0 = RidgedMulti::new()
            .set_seed(*CURRENT_SEED + 30)
            .set_frequency(1723.0)
            .set_lacunarity(*MOUNTAIN_LACUNARITY)
            .set_octaves(4);

        // Step 2: Identify rocky mountain ranges.
        // Using the scale/shift function, scale the defined mountain ranges
        // obtained in step 1 so that its ridges are not too high.
        // I do this so that later, in another subgroup, I can add the actual
        // mountainous terrain to these ridges.
        let mountain_base_def_sb0: ScaleBias<[f64; 3]> = ScaleBias::new(&mountain_base_def_rm0);
        let mountain_base_def_sb0 = mountain_base_def_sb0.set_scale(0.5).set_bias(0.375);

        // Step 3: Determine river valleys.
        // I use the ribbed multifractal noise function to generate river valleys.
        // I apply much lower frequency than for generation of mountain ranges.
        // This is necessary so that more mountain ranges appear outside the valley.
        // It is important to record that this noise function generates a
        // ribbed-multifractal noise using only one octave
        let mountain_base_def_rm1 = RidgedMulti::new().set_seed(*CURRENT_SEED + 31);
        let mountain_base_def_rm1 = mountain_base_def_rm1
            .set_frequency(367.0)
            .set_lacunarity(*MOUNTAIN_LACUNARITY)
            .set_octaves(1);

        // Step 4: Scaling of river valleys.
        // I use the scale/offset function and apply the scaling factor -2.0
        // for the output value of step 3.
        // This stretches the possible pitch values because the rib is one octave higher
        // of multifractal noise has a smaller range of output values than the
        // rib-to-multifractal noise.
        // A negative scaling factor inverts the output value range,
        // rotating the ridges from the output value of step 3.
        let mountain_base_def_sb1: ScaleBias<[f64; 3]> = ScaleBias::new(&mountain_base_def_rm1);
        let mountain_base_def_sb1 = mountain_base_def_sb1.set_scale(-2.0).set_bias(-0.5);

        // Step 5. Create a constant
        let mountain_base_def_co = Constant::new(-1.0);

        // Step 6: Mountains and valleys.
        // I combine the results of step 3 and step 4 in a blender. This results in
        // the low-lying areas are smooth, and the upland areas have ridges.
        // terrain contain ridges. The result of step 4 is used for this purpose.
        // as a control module.
        let mountain_base_def_bl: Blend<[f64; 3]> = Blend::new(
            &mountain_base_def_co,
            &mountain_base_def_sb0,
            &mountain_base_def_sb1,
        );

        // Step 7. Rough turbulence
        // Using turbulence I distort the result from step 6 by adding
        // random coarse details to it.
        let mountain_base_def_tu0 = Turbulence::<_>::new(&mountain_base_def_bl)
            .set_seed(*CURRENT_SEED + 32)
            .set_frequency(1337.0)
            .set_power(1.0 / 6730.0 * *MOUNTAINS_TWIST)
            .set_roughness(4);

        // Step 8: Warp mountains and peaks.
        // Using turbulence warp the result of coarse turbulence.
        // This turbulence has higher frequencies but less power,
        // than the coarse turbulence. This adds random fine detail.
        let mountain_base_def_tu1: Turbulence<&Turbulence<&Blend<[f64; 3]>>> =
            Turbulence::<_>::new(&mountain_base_def_tu0)
                .set_seed(*CURRENT_SEED + 33)
                .set_frequency(21221.0)
                .set_power(1.0 / 120157.0 * *MOUNTAINS_TWIST)
                .set_roughness(6);

        // Final step of the subgroup.
        // Kesh the obtained result of curved mountains and valleys.
        let mountain_base_def = Cache::new(&mountain_base_def_tu1);

        /////////////////////////////////////////////////////////////////////////////
        // Subgroup: High Mountain Terrain
        /////////////////////////////////////////////////////////////////////////////
        //
        // This subgroup generates mountainous terrain that appears at the height of the
        // elevation within mountain ranges.
        //
        // -1.0 represents the smoothest terrain types (plains and underwater) and
        // +1.0 represents the most rugged terrain types (mountains).
        /////////////////////////////////////////////////////////////////////////////

        // Step 1: Generate Mountains.
        // I use the multifractal noise function.
        let mountainous_high_rm0 = RidgedMulti::new().set_seed(*CURRENT_SEED + 40);
        let mountainous_high_rm0 = mountainous_high_rm0
            .set_frequency(2371.0)
            .set_lacunarity(*MOUNTAIN_LACUNARITY)
            .set_octaves(3);

        let mountainous_high_rm1 = RidgedMulti::new().set_seed(*CURRENT_SEED + 41);
        let mountainous_high_rm1 = mountainous_high_rm1
            .set_frequency(2341.0)
            .set_lacunarity(*MOUNTAIN_LACUNARITY)
            .set_octaves(3);

        // Step 2: Highlands
        // Trying to generate more mountains at the expense of valleys. I do this by
        // providing maximum output values from two ribbed functions
        // the multifractal noise of step 1.
        let mountainous_high_ma: Max<[f64; 3]> =
            Max::new(&mountainous_high_rm0, &mountainous_high_rm1);

        // Step 3: Distort the highlands.
        // I use turbulence and add random details.
        let mountainous_high_tu = Turbulence::<_>::new(&mountainous_high_ma)
            .set_seed(*CURRENT_SEED + 42)
            .set_frequency(31511.0)
            .set_power(1.0 / 180371.0 * *MOUNTAINS_TWIST)
            .set_roughness(4);

        // Final step of the subgroup.
        // Cache the result of this subgroup execution.
        let mountainous_high = Cache::new(mountainous_high_tu);

        /////////////////////////////////////////////////////////////////////////////
        // Subgroup: Lowlands/Low Mountainous Areas
        /////////////////////////////////////////////////////////////////////////////
        //
        // This subgroup generates upland terrain that appears at low
        // elevations in river valleys.
        //
        // -1.0 represents the smoothest terrain types (plains and underwater) and
        // +1.0 represents the most rugged terrain types (mountains).
        /////////////////////////////////////////////////////////////////////////////

        // Step 1: Basis of lowlands
        // I use the ribbed multifractal noise function, generating the
        // lowland terrain.
        let mountainous_low_rm0 = RidgedMulti::new()
            .set_seed(*CURRENT_SEED + 50)
            .set_frequency(1381.0)
            .set_lacunarity(*MOUNTAIN_LACUNARITY)
            .set_octaves(8);

        let mountainous_low_rm1 = RidgedMulti::new()
            .set_seed(*CURRENT_SEED + 51)
            .set_frequency(1427.0)
            .set_lacunarity(*MOUNTAIN_LACUNARITY)
            .set_octaves(8);

        // Step 2: Create a lowland landscape.
        // I use multiplication to combine the two functions of the comb
        // multifractal noise values from step 1.
        // This causes the following landscape to appear:
        //
        // - Cracks - appear when two negative output values are multiplied.
        //
        // - Flat areas - appear when positive and
        // negative output values are multiplied.
        //
        // - Ribs - appear when two positive output values are multiplied.
        let mountainous_low_mu: Multiply<[f64; 3]> =
            Multiply::new(&mountainous_low_rm0, &mountainous_low_rm1);

        // Final step of the subgroup.
        // Cache the result of this subgroup execution.
        let mountainous_low = Cache::new(&mountainous_low_mu);

        /////////////////////////////////////////////////////////////////////////////
        // Subgroup: Mountainous terrain
        /////////////////////////////////////////////////////////////////////////////
        //
        // This subgroup generates the final mountainous terrain by combining the
        // the result of the generation from the highlands and lowlands.
        //
        // -1.0 represents the smoothest terrain types (plains and underwater) and
        // +1.0 represents the roughest terrain types (mountains).
        /////////////////////////////////////////////////////////////////////////////

        // Step 1: Scale low-altitude terrain.
        // Scale the output value from the `Lowland/Low Terrain' subgroup
        // to a very low value and shift it towards -1.0. As a result
        // low-altitude terrain becomes flatter with small elevations.
        // Likewise. low mountains appear in the lowest areas of a given terrain.
        let mountainous_terrain_sb0 = ScaleBias::new(&mountainous_low)
            .set_scale(0.03125)
            .set_bias(-0.96875);

        // Step 2: Scaling of high-mountainous terrain.
        // I scale the output value from the `Highlands` subgroup to 1/4
        // of its initial output value and shift it so that its output
        // value is usually positive.
        let mountainous_terrain_sb1 = ScaleBias::new(&mountainous_high)
            .set_scale(0.25)
            .set_bias(0.25);

        // Step 3: Some more mountains.
        // Derive an additional value from step 2 so that the mountains appear
        // all over the terrain.
        let mountainous_terrain_ad: Add<[f64; 3]> =
            Add::new(&mountainous_terrain_sb1, &mountain_base_def);

        // Step 3.
        // Since the entire terrain is now covered by highland terrain, even at low altitudes.
        // The appearance of highlands should be limited to the tops of elevations only.
        // I create a mapping of the highlands to the lowlands.
        // I do this with a noise function that selects the output value from the subgroup of
        // mountainous terrain, if the output value from the mountain base is higher than the set sum.
        // Otherwise, this noise selects the output value from the scaled lowland terrain.
        let mountainous_terrain_se = Select::new(
            &mountainous_terrain_sb0,
            &mountainous_terrain_ad,
            &mountain_base_def,
        )
        .set_bounds(-0.5, 999.5)
        .set_falloff(0.5);

        // Step 4: Scaling the mountainous terrain.
        // I slightly reduce the range of the output value of step 3, reducing the height of the
        // mountain peaks.
        let mountainous_terrain_sb2 = ScaleBias::new(&mountainous_terrain_se)
            .set_scale(0.8)
            .set_bias(0.0);

        // Step 5: Ice coating.
        //
        // TODO: THERE'S A BUG
        //
        // I apply an exponential curve to the output value of step 4.
        // This causes the mountainsides to increase smoothly to higher elevations,
        // as if a glacier is grinding down these mountains.
        //
        // The exponential curve function expects an output value between -1.0 and +1.0.
        let mountainous_terrain_ex =
            Exponent::new(&mountainous_terrain_sb2).set_exponent(*MOUNTAIN_GLACIATION);

        // Final step for all groups.
        // Cache the obtained intermediate result.
        let mountainous_terrain = Cache::new(&mountainous_terrain_ex);
        
        /////////////////////////////////////////////////////////////////////////////
        // Band of Steps: HILLS.
        /////////////////////////////////////////////////////////////////////////////

        /////////////////////////////////////////////////////////////////////////////
        // Subgroup: Hilly terrain
        /////////////////////////////////////////////////////////////////////////////
        //
        // This subgroup generates hilly terrain.
        //
        // -1.0 represents the smoothest terrain types (plains and underwater) and
        // +1.0 represents the roughest terrain types (mountains).
        /////////////////////////////////////////////////////////////////////////////

        // Step 1: Generate hills using the wave function
        let hilly_terrain_bi = Billow::new()
            .set_seed(*CURRENT_SEED + 60)
            .set_frequency(1663.0)
            .set_persistence(0.5)
            .set_lacunarity(*HILLS_LACUNARITY)
            .set_octaves(6);

        // Step 2: Apply scaling/offset to the result from step 1.
        // This removes too high vertices.
        let hilly_terrain_sb0: ScaleBias<[f64; 3]> = ScaleBias::new(&hilly_terrain_bi);
        let hilly_terrain_sb0 = hilly_terrain_sb0.set_scale(0.5).set_bias(0.5);

        // Step 3: Create river valleys.
        // I use the ribbed multifractal noise function to generate river valleys.
        // I set a much lower frequency so that more hills appear between the valleys.
        let hilly_terrain_rm = RidgedMulti::new()
            .set_seed(*CURRENT_SEED + 61)
            .set_frequency(367.5)
            .set_lacunarity(*HILLS_LACUNARITY)
            .set_octaves(1);

        // Step 4: I apply a scaling factor of -2.0 to the output value of step 3.
        // This stretches the possible height values, because the single-octave
        // ribbed-multifractal noise has a smaller range of output values,
        // than multi-octave ribbed-multifractal noise. The negative coefficient
        // scaling inverts the range of the output value by rotating the ridges
        // from river valleys into valleys.
        let hilly_terrain_sb1: ScaleBias<[f64; 3]> = ScaleBias::new(&hilly_terrain_rm);
        let hilly_terrain_sb1 = hilly_terrain_sb1.set_scale(-2.0).set_bias(-1.0);

        let hilly_terrain_co = Constant::new(-1.0);

        // Step 5: Combine the result of hills and river valleys generation.
        // This causes lowland areas to become smooth,
        // and upland areas contain ridges.
        let hilly_terrain_bl: Blend<[f64; 3]> =
            Blend::new(&hilly_terrain_co, &hilly_terrain_sb1, &hilly_terrain_sb0);

        // Step 6: Using the scale/shift function, I slightly reduce the range of the
        // of the output value from step 5, which allows to reduce the height of the hilltops.
        let hilly_terrain_sb2: ScaleBias<[f64; 3]> = ScaleBias::new(&hilly_terrain_bl)
            .set_scale(0.75)
            .set_bias(-0.25);

        // Step 7: Increase the slope of hills.
        // At higher altitudes, this exponential curve function is applied
        // to the output value from step 6. This exponential curve function expects,
        // that the input value will be between -1.0 and 1.0.
        let hilly_terrain_ex = Exponent::new(&hilly_terrain_sb2).set_exponent(1.375);

        // Step 8: Applying turbulence to add coarse details
        // for the output value from step 7.
        let hilly_terrain_tu0: Turbulence<&Exponent<[f64; 3]>> = Turbulence::new(&hilly_terrain_ex)
            .set_seed(*CURRENT_SEED + 62)
            .set_frequency(1531.0)
            .set_power(1.0 / 16921.0 * *HILLS_TWIST)
            .set_roughness(4);

        // Step 9: Apply turbulence to add fine detail.
        // Set a higher frequency but lower power, relative to step 8.
        let hilly_terrain_tu1 = Turbulence::<_>::new(&hilly_terrain_tu0)
            .set_seed(*CURRENT_SEED + 63)
            .set_frequency(21617.0)
            .set_power(1.0 / 117529.0 * *HILLS_TWIST)
            .set_roughness(6);

        // Final step for the whole group
        // Caching of the current intermediate result.
        let hilly_terrain = Cache::new(hilly_terrain_tu1);

        /////////////////////////////////////////////////////////////////////////////
        // Группа шагов: РАВНИНЫ
        /////////////////////////////////////////////////////////////////////////////

        /////////////////////////////////////////////////////////////////////////////
        // Подгруппа: Равнинная местность
        /////////////////////////////////////////////////////////////////////////////
        //
        // Эта подгруппа генерирует равнинный ландшафт.
        // Поскольку эта подгруппа в конечном итоге будет значительно сплющена,
        // типы и комбинации шумовых функций, которые генерируют равнины,
        // не имеют значительного эффекта на конечный результат.
        //
        // -1.0 представляет самые гладкие типы местности (равнины и под водой) и
        // +1.0 представляет самые пересеченные типы местности (горы).
        /////////////////////////////////////////////////////////////////////////////

        // Шаг 1. Создание равнин через функцию шумоподавления.
        let plains_terrain_bi0 = Billow::new()
            .set_seed(*CURRENT_SEED + 70)
            .set_frequency(1097.5)
            .set_persistence(0.5)
            .set_lacunarity(*PLAINS_LACUNARITY)
            .set_octaves(8);

        // Шаг 2. Применяю функцию масштаба/смещения, чтобы выходное значение
        // шага 1 было положительным.
        let plains_terrain_sb0: ScaleBias<[f64; 3]> = ScaleBias::new(&plains_terrain_bi0);
        let plains_terrain_sb0 = plains_terrain_sb0.set_scale(0.5).set_bias(0.5);

        // Шаг 3. Еще раз шумоподавление.
        let plains_terrain_bi1 = Billow::new()
            .set_seed(*CURRENT_SEED + 71)
            .set_frequency(1097.5)
            .set_persistence(0.5)
            .set_lacunarity(*PLAINS_LACUNARITY)
            .set_octaves(8);

        // Шаг 4. Применяю функцию масштаба/смещения, чобы получить положительный результат.
        let plains_terrain_sb1: ScaleBias<[f64; 3]> = ScaleBias::new(&plains_terrain_bi1);
        let plains_terrain_sb1 = plains_terrain_sb1.set_scale(0.5).set_bias(0.5);

        // Шаг 5. Объединение базового реультата и равнин.
        let plains_terrain_mu: Multiply<[f64; 3]> =
            Multiply::new(&plains_terrain_sb0, &plains_terrain_sb1);

        // Шаг 6.
        // Применяю функцию масштаба/смещения, которая преобразует выходное значение
        // в диапазоне от 0,0 до 1,0 обратно в значение в диапазоне от -1,0 до +1,0.
        let plains_terrain_sb2: ScaleBias<[f64; 3]> = ScaleBias::new(&plains_terrain_mu)
            .set_scale(2.0)
            .set_bias(-1.0);

        // Финальный шаг для всей группы
        // Кеширование промежуточного результата
        let plains_terrain: Cache<ScaleBias<[f64; 3]>> = Cache::new(plains_terrain_sb2);

        /////////////////////////////////////////////////////////////////////////////
        // Группа шагов: БЕСПЛОДНЫЕ ЗЕМЛИ
        /////////////////////////////////////////////////////////////////////////////

        /////////////////////////////////////////////////////////////////////////////
        // Подгруппа: Песок бесплодных земель
        /////////////////////////////////////////////////////////////////////////////
        //
        // Эта подгруппа генерирует песчаный ландшафт для бесплодных земель.
        //
        // -1.0 представляет самые гладкие типы местности (равнины и под водой) и
        // +1.0 представляет самые пересеченные типы местности (горы).
        /////////////////////////////////////////////////////////////////////////////

        // Шаг 1. Применяю функцию ребристого мультифрактального шума для генерации
        // песчаных дюн. Использую однооктавный шум для создания гладких дюн.
        let badlands_sand_rm = RidgedMulti::new()
            .set_seed(*CURRENT_SEED + 80)
            .set_frequency(6163.5)
            .set_lacunarity(*BADLANDS_LACUNARITY)
            .set_octaves(1);

        // Шаг 2. Создание чешуйчатых дюн.
        // Уменьшаю высоту дюн на небольшую величину. Это необходимо, чтобы потом можно
        // было добавить случайнве детали к дюнам.
        let badlands_sand_sb0: ScaleBias<[f64; 3]> = ScaleBias::new(&badlands_sand_rm);
        let badlands_sand_sb0 = badlands_sand_sb0.set_scale(0.875).set_bias(0.0);

        // Шаг 3. Применяю полигоны Вороного для генерации деталей в дюнах.
        // Создаются небольшие полигональные ямы, их края сиеденяются с краями ближайших ям.
        let badlands_sand_wo = Worley::new()
            .set_seed(*CURRENT_SEED + 81)
            .set_frequency(16183.25);

        // Шаг 4. Через функцию масштабирования/смещения значительно уменьшаю детали дюн.
        let badlands_sand_sb1: ScaleBias<[f64; 3]> = ScaleBias::new(&badlands_sand_wo);
        let badlands_sand_sb1 = badlands_sand_sb1.set_scale(0.25).set_bias(0.25);

        // Шаг 5. Объединяю дюны с деталями с обычными дюнами.
        let badlands_sand_ad = Add::new(&badlands_sand_sb0, &badlands_sand_sb1);

        // Финальный шаг подгруппы
        // Кеширование промежуточного результата
        let badlands_sand: Cache<&Add<[f64; 3]>> = Cache::new(&badlands_sand_ad);

        /////////////////////////////////////////////////////////////////////////////
        // Подгруппа: Скалы бесплодных земель
        /////////////////////////////////////////////////////////////////////////////
        //
        // Эта подгруппа генерирует скалы для бесплодных земель.
        //
        // -1.0 представляет самые гладкие типы местности (равнины и под водой) и
        // +1.0 представляет самые пересеченные типы местности (горы).
        /////////////////////////////////////////////////////////////////////////////

        // Шаг 1. Генерирую когерентный шум, чтобы потом с помощью него создавать обрывы.
        let badlands_cliffs_fb = Fbm::new()
            .set_seed(*CURRENT_SEED + 90)
            .set_frequency(*CONTINENT_FREQUENCY * 839.0)
            .set_persistence(0.5)
            .set_lacunarity(*BADLANDS_LACUNARITY)
            .set_octaves(6);

        // Шаг 2. Применяю функцию кривой к результату шага 1.
        // Кривая изначально пологая, но затем ее наклон резко возрастает.
        // На самом высоком высоты кривая снова становится очень плоской.
        // Это создает обвесные пустынные скалы.
        let badlands_cliffs_cu: Curve<[f64; 3]> = Curve::new(&badlands_cliffs_fb);
        let badlands_cliffs_cu = badlands_cliffs_cu
            .add_control_point(-2.000, -2.000)
            .add_control_point(-1.000, -1.000)
            .add_control_point(-0.000, -0.750)
            .add_control_point(0.500, -0.250)
            .add_control_point(0.625, 0.875)
            .add_control_point(0.750, 1.000)
            .add_control_point(2.000, 1.250);

        // Шаг 3. Делаю плоскими обрывы полученные на шаге 2.
        let badlands_cliffs_cl = Clamp::new(&badlands_cliffs_cu).set_bounds(-999.125, 0.875);

        // Шаг 4. Создание терассных скал, на основе шага 3.
        // Скалы в нижних отметках будут иметь резкий обрыв.
        let badlands_cliffs_te: Terrace<[f64; 3]> = Terrace::new(&badlands_cliffs_cl)
            .add_control_point(-1.000)
            .add_control_point(-0.875)
            .add_control_point(-0.750)
            .add_control_point(-0.500)
            .add_control_point(0.000)
            .add_control_point(1.000);

        // Шаг 5. Турбулентностью искажаю результат шага 4,
        // Добавляю к нему случайные грубые детали.
        let badlands_cliffs_tu0: Turbulence<&Terrace<[f64; 3]>> =
            Turbulence::new(&badlands_cliffs_te)
                .set_seed(*CURRENT_SEED + 91)
                .set_frequency(16111.0)
                .set_power(1.0 / 141539.0 * *BADLANDS_TWIST)
                .set_roughness(3);

        // Шаг 6. Искривление скал
        //  Искажаю резултьтат грубой турбулентности, добавляя мелкие детали.
        let badlands_cliffs_tu1: Turbulence<&Turbulence<&Terrace<[f64; 3]>>> =
            Turbulence::new(&badlands_cliffs_tu0)
                .set_seed(*CURRENT_SEED + 92)
                .set_frequency(36107.0)
                .set_power(1.0 / 211543.0 * *BADLANDS_TWIST)
                .set_roughness(3);

        // Финальный шаг подгруппы
        // Кеширование промежуточного результата
        let badlands_cliffs = Cache::new(&badlands_cliffs_tu1);

        /////////////////////////////////////////////////////////////////////////////
        // Подгруппа: Ландшафт бесплодных земель
        /////////////////////////////////////////////////////////////////////////////
        //
        // Генерирует окончательный ландшафт бесплодных земель.
        // Выравниваю песок бесплодных земель, затем снижаю высоту песка до -1.0.
        // Делаю появление песка тольклько на низких высотах.
        //
        // -1.0 представляет самые гладкие типы местности (равнины и под водой) и
        // +1.0 представляет самые пересеченные типы местности (горы).
        /////////////////////////////////////////////////////////////////////////////

        // Шаг 1. Ровняю результат подгруппы бесплодных земель.
        // Понижаю значение близко к -1.0.
        let badlands_terrain_sb = ScaleBias::new(&badlands_sand)
            .set_scale(0.25)
            .set_bias(-0.75);

        // Шаг 2. Создаю условия, чтобы дюны появлялись только в низких облостях,
        // а скалы только в высоких.
        let badlands_terrain_ma: Max<[f64; 3]> = Max::new(&badlands_cliffs, &badlands_terrain_sb);

        // Финальный шаг группы
        // Кеширование промежуточного результата
        let badlands_terrain: Cache<&Max<[f64; 3]>> = Cache::new(&badlands_terrain_ma);

        /////////////////////////////////////////////////////////////////////////////
        // Группа шагов: РЕКИ
        /////////////////////////////////////////////////////////////////////////////

        /////////////////////////////////////////////////////////////////////////////
        // Подгруппа: Позиция рек
        /////////////////////////////////////////////////////////////////////////////
        //
        // Эта подгруппа генерирует положение рек на текущей местности.
        //
        // -1.0 представляет самые гладкие типы местности (равнины и под водой) и
        // +1.0 представляет самые пересеченные типы местности (горы).
        /////////////////////////////////////////////////////////////////////////////

        // Шаг 1. Создание широких и глубоких рек через применение
        // ребристого мультифрактального шума.
        let river_positions_rm0 = RidgedMulti::new()
            .set_seed(*CURRENT_SEED + 100)
            .set_frequency(18.75)
            .set_lacunarity(*CONTINENT_LACUNARITY)
            .set_octaves(1);

        // Шаг 2. Инвертирую горные хребты и снижаю край рек, создавая
        // резкий переход от суши к реке.
        let river_positions_cu0: Curve<[f64; 3]> = Curve::new(&river_positions_rm0);
        let river_positions_cu0 = river_positions_cu0
            .add_control_point(-2.000, 2.000)
            .add_control_point(-1.000, 1.000)
            .add_control_point(-0.125, 0.875)
            .add_control_point(0.000, -1.000)
            .add_control_point(1.000, -1.500)
            .add_control_point(2.000, -2.000);

        // Шаг 3. Создание неглубоких рек через использование
        // ребристого мультифрактального шума.
        let river_positions_rm1 = RidgedMulti::new()
            .set_seed(*CURRENT_SEED + 101)
            .set_frequency(43.25)
            .set_lacunarity(*CONTINENT_LACUNARITY)
            .set_octaves(1);

        // Шаг 4. Повторяю операции из шага 2 для шага 3.
        let river_positions_cu1: Curve<[f64; 3]> = Curve::new(&river_positions_rm1);
        let river_positions_cu1 = river_positions_cu1
            .add_control_point(-2.000, 2.0000)
            .add_control_point(-1.000, 1.5000)
            .add_control_point(-0.125, 1.4375)
            .add_control_point(0.000, 0.5000)
            .add_control_point(1.000, 0.2500)
            .add_control_point(2.000, 0.0000);

        // Шаг 5. Объединение больших рек с маленькими.
        let river_positions_mi: Min<[f64; 3]> =
            Min::new(&river_positions_cu0, &river_positions_cu1);

        // Шаг 6. Искажаю выходное значение полученное в результате комбинирования типов рек.
        // Это немного скручивает реки и добовляет шероховатости в шум.
        let river_positions_tu: Turbulence<&Min<[f64; 3]>> =
            Turbulence::<_>::new(&river_positions_mi)
                .set_seed(*CURRENT_SEED + 102)
                .set_frequency(9.25)
                .set_power(1.0 / 57.75)
                .set_roughness(6);

        // Финальный шаг подгруппы
        // Кеширование промежуточного результата
        let river_positions: Cache<&Turbulence<&Min<[f64; 3]>>> = Cache::new(&river_positions_tu);

        /////////////////////////////////////////////////////////////////////////////
        // Группа шагов: ГОРНЫЙ РЕЛЬЕФ
        /////////////////////////////////////////////////////////////////////////////

        /////////////////////////////////////////////////////////////////////////////
        // Подгруппа: Масштабированный горный рельеф
        /////////////////////////////////////////////////////////////////////////////
        //
        // Эта подгруппа масштабирует выходное значение из группы горной местности
        // чтобы его можно было добавить к высоте, определяемой континентом.
        // Подгруппа масштабирует выходное значение таким образом, чтобы оно почти всегда
        // было положительным. Делаю это для того, чтобы не применялась отрицательная отметка
        // к группе определяющей континенты.
        //
        // Выходное значение этой подгруппы модулей измеряется в планетарных где:
        // -1.0 представляет самые гладкие типы местности (равнины и под водой) и
        // +1.0 представляет самые пересеченные типы местности (горы).
        /////////////////////////////////////////////////////////////////////////////

        // Шаг 1. Функцией масштаба/смещения масштабирую выходное значение полученное
        // в греппе генерации гор так, чтобы выходное значение измеряется в п
        // ланетарных единицах высоты.
        let scaled_mountainous_terrain_sb0 = ScaleBias::new(&mountainous_terrain)
            .set_scale(0.125)
            .set_bias(0.125);

        // Шаг 2. На данном этапе высота гор везде линейно равна. Поэтому создаю шум
        // который в дальнейшем использую в шумовых функциях для рандомизации горных высот.
        let scaled_mountainous_terrain_fb = Fbm::new()
            .set_seed(*CURRENT_SEED + 110)
            .set_frequency(14.5)
            .set_persistence(0.5)
            .set_lacunarity(*MOUNTAIN_LACUNARITY)
            .set_octaves(6);

        // Шаг 3. Создаю экспоненциальную кривую которую применяю к шагу 2. Это дает
        // небольшое количество высоких значений и гораздо большее количество низких значений.
        // Это гарантирует, что в мире будет существовать несколько высоких горных пиков,
        // чем большинство остальных горных массивов. Это дает более разнообразный ландшафт.
        let scaled_mountainous_terrain_ex: Exponent<[f64; 3]> =
            Exponent::new(&scaled_mountainous_terrain_fb);
        let scaled_mountainous_terrain_ex = scaled_mountainous_terrain_ex.set_exponent(1.25);

        // Шаг 4. Применяю функцию масштабирования/смещения для шага 3. Это необходимо
        // чтобы это выходное значение было не намного меньше 1,0.
        let scaled_mountainous_terrain_sb1 = ScaleBias::new(&scaled_mountainous_terrain_ex)
            .set_scale(0.25)
            .set_bias(1.0);

        // Шаг 5. Моделирую высоту горных вершин на основе генерации горных пиков
        let scaled_mountainous_terrain_mu = Multiply::new(
            &scaled_mountainous_terrain_sb0,
            &scaled_mountainous_terrain_sb1,
        );

        // Финальный шаг подгруппы
        // Кеширование промежуточного результата. Это выходное значение для всей горной местности.
        let scaled_mountainous_terrain = Cache::new(&scaled_mountainous_terrain_mu);

        /////////////////////////////////////////////////////////////////////////////
        // Группа шагов: МАСШТАБИРУЕМАЯ ХОЛМИСТАЯ МЕСТНОСТЬ
        /////////////////////////////////////////////////////////////////////////////

        /////////////////////////////////////////////////////////////////////////////
        // Подгруппа: Масштабируемая холмистая местность
        /////////////////////////////////////////////////////////////////////////////
        //
        // Эта подгруппа масштабирует выходное значение из группы холмистой местност.
        // Величина масштабирования, применяемая к холмам, составляет половину величины
        // масштабирования от примененной к к группе масштабируемой горной местности.
        //
        // Подгруппа масштабирует выходное значение таким образом, чтобы оно почти всегда
        // было положительным. Это сделано для того, чтобы отрицательные отметки не применялись
        // к группе определенных континентов.
        //
        // Выходное значение этой подгруппы модулей измеряется в планетарных где:
        // -1.0 представляет самые гладкие типы местности (равнины и под водой) и
        // +1.0 представляет самые пересеченные типы местности (горы).
        /////////////////////////////////////////////////////////////////////////////

        // Шаг 1. Через функицю масштаба/смещения масштабирую выходное значение из группы
        // холмистой местности, чтобы это выходное значение было измеряется в
        // планетарных единицах высоты.
        let scaled_hilly_terrain_sb0 = ScaleBias::new(&hilly_terrain)
            .set_scale(0.0625)
            .set_bias(0.0625);

        // Шаг 2. Высотах всех холмов примерна одинакома, посему применяю шумовую функцию
        // для создания разной высоты у холмов.
        let scaled_hilly_terrain_fb = Fbm::new()
            .set_seed(*CURRENT_SEED + 120)
            .set_frequency(13.5)
            .set_persistence(0.5)
            .set_lacunarity(*HILLS_LACUNARITY)
            .set_octaves(6);

        // Шаг 3. Добавляю больше разнообразия. В мире должно появиться немного высоких холмов
        // и много разных, но с более низким показателем по высоте.
        let scaled_hilly_terrain_ex: Exponent<[f64; 3]> = Exponent::new(&scaled_hilly_terrain_fb);
        let scaled_hilly_terrain_ex = scaled_hilly_terrain_ex.set_exponent(1.25);

        // Шаг 4. Применяю функцию масштаба/смещения для выходного значения из шага 3.
        let scaled_hilly_terrain_sb1 = ScaleBias::new(&scaled_hilly_terrain_ex)
            .set_scale(0.5)
            .set_bias(1.5);

        // Шаг 5. Еще немного преобразований для значений из шага 4.
        let scaled_hilly_terrain_mu =
            Multiply::new(&scaled_hilly_terrain_sb0, &scaled_hilly_terrain_sb1);

        // Финальный шаг группы
        // Кеширование промежуточного результата. Это выходное значение для всей холмистой местности.
        let scaled_hilly_terrain = Cache::new(&scaled_hilly_terrain_mu);
    }
}
