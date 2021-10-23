use core::fmt;
use deku::prelude::*;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum Version {
    StandardScheme = 1,
    SuperWeaponScheme = 2,
    ExtendedScheme = 3,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(magic = b"SCHM")]
struct Header {
    version: Version,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum StockpilingMode {
    Off = 0,
    On = 1,
    Anti = 2,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum WormSelect {
    Sequential = 0,
    On = 1,
    Random = 2,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum SuddenDeathEvent {
    RoundEnds = 0,
    NuclearStrike = 1,
    HPDrops = 2,
    Nothing = 3,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
struct Options {
    hot_seat_delay: u8,
    retreat_time: u8,
    rope_retreat_time: u8,
    display_total_round_time: bool,
    automatic_replays: bool,
    fall_damage: u8,
    artillery_mode: bool,
    bounty_mode: bool,
    stockpiling_mode: StockpilingMode,
    worm_select: WormSelect,
    sudden_death_event: SuddenDeathEvent,
    water_rise_rate: u8,
    weapon_crate_probability: i8,
    donor_cards: bool,
    health_crate_probability: i8,
    health_crate_energy: u8,
    utility_crate_probability: i8,
    hazardous_object_types: u8,
    mine_delay: i8,
    dud_mines: bool,
    manual_worm_placement: bool,
    initial_worm_energy: u8,
    turn_time: i8,
    round_time: i8,
    number_of_wins: u8,
    blood: bool,
    aqua_sheep: bool,
    sheep_heaven: bool,
    god_worms: bool,
    indestructible_land: bool,
    upgraded_grenade: bool,
    upgraded_shotgun: bool,
    upgraded_clusters: bool,
    upgraded_longbow: bool,
    team_weapons: bool,
    super_weapons: bool,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
struct Weapon {
    ammunition: u8,
    power: u8,
    delay: u8,
    probability: u8,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
struct Weapons {
    bazooka: Weapon,
    homing_missile: Weapon,
    mortar: Weapon,
    grenade: Weapon,
    cluster_bomb: Weapon,
    skunk: Weapon,
    petrol_bomb: Weapon,
    banana_bomb: Weapon,
    handgun: Weapon,
    shotgun: Weapon,
    uzi: Weapon,
    minigun: Weapon,
    longbow: Weapon,
    airstrike: Weapon,
    napalm_strike: Weapon,
    mine: Weapon,
    fire_punch: Weapon,
    dragon_ball: Weapon,
    kamikaze: Weapon,
    prod: Weapon,
    battle_axe: Weapon,
    blowtorch: Weapon,
    pneumatic_drill: Weapon,
    girder: Weapon,
    ninja_rope: Weapon,
    parachute: Weapon,
    bungee: Weapon,
    teleport: Weapon,
    dynamite: Weapon,
    sheep: Weapon,
    baseball_bat: Weapon,
    flame_thrower: Weapon,
    homing_pigeon: Weapon,
    mad_cow: Weapon,
    holy_hand_grenade: Weapon,
    old_woman: Weapon,
    sheep_launcher: Weapon,
    super_sheep: Weapon,
    mole_bomb: Weapon,
    jet_pack: Weapon,
    low_gravity: Weapon,
    laser_sight: Weapon,
    fast_walk: Weapon,
    invisibility: Weapon,
    damage_x2: Weapon,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
struct SuperWeapons {
    freeze: Weapon,
    super_banana_bomb: Weapon,
    mine_strike: Weapon,
    girder_starter_pack: Weapon,
    earthquake: Weapon,
    scales_of_justice: Weapon,
    ming_vase: Weapon,
    mikes_carpet_bomb: Weapon,
    patsys_magic_bullet: Weapon,
    indian_nuclear_test: Weapon,
    select_worm: Weapon,
    salvation_army: Weapon,
    mole_squadron: Weapon,
    mb_bomb: Weapon,
    concrete_donkey: Weapon,
    suicide_bomber: Weapon,
    sheep_strike: Weapon,
    mail_strike: Weapon,
    armageddon: Weapon,
}

#[derive(Serialize, Deserialize, DekuRead, DekuWrite)]
struct FixedPoint(i32);

impl fmt::Debug for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", (self.0 as f32) / 65536.0)
    }
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum PhasedWorms {
    Off = 0,
    Worms = 1,
    WormsWeapons = 2,
    WormsWeaponsDamage = 3,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum RopeRollDrops {
    Disabled = 0,
    AsFromRopeOnly = 1,
    AsFromRopeOrJump = 2,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum KeepControlAfterSkimming {
    LoseControl = 0,
    KeepControl = 1,
    KeepControlAndRope = 2,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum TriState {
    False = 0,
    True = 1,
    Default = 0x80,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum Skipwalking {
    Disabled = 0xFF,
    Possible = 0,
    Facilitated = 1,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum BlockRoofing {
    Allow = 0,
    BlockAbove = 1,
    BlockEverywhere = 2,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum RubberWormGravityType {
    Unmodified = 0,
    Standard = 1,
    BlackHoleConstant = 2,
    BlackHoleLinear = 3,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
struct RubberwormOptions {
    bounciness: FixedPoint,
    air_viscosity: FixedPoint,
    air_viscosity_applies_to_worms: bool,
    wind_influence: FixedPoint,
    wind_influence_applies_to_worms: bool,
    gravity_type: RubberWormGravityType,
    gravity_strength: FixedPoint,
    crate_rate: u8,
    crate_shower: bool,
    anti_sink: bool,
    remember_weapons: bool,
    extended_fuses_herds: bool,
    anti_lock_aim: bool,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(type = "u8")]
enum HealthCratesCurePoison {
    Disabled = 0xFF,
    CollectingWorm = 0,
    CollectingWormTeam = 1,
    CollectingWormTeamsAllied = 2,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
struct ExtendedOptions {
    data_version: u32,
    constant_wind: bool,
    wind: i16,
    wind_bias: u8,
    gravity: FixedPoint,
    terrain_friction: FixedPoint,
    rope_knocking: u8,
    blood_level: u8,
    unrestrict_rope: bool,
    auto_place_worms_by_ally: bool,
    no_crate_probability: u8,
    maximum_crate_count_on_map_at_once: u16,
    sudden_death_disables_worm_select: bool,
    sudden_death_worm_damage_per_turn: u8,
    phased_worms_allied: PhasedWorms,
    phased_worms_enemy: PhasedWorms,
    circular_aim: bool,
    anti_lock_aim: bool,
    worm_selection_doesnt_end_hot_seat: bool,
    worm_selection_is_never_cancelled: bool,
    batty_rope: bool,
    rope_roll_drops: RopeRollDrops,
    x_impact_loss_of_control: u8, // This is a weird enum
    keep_control_after_bumping_head: bool,
    keep_control_after_skimming: KeepControlAfterSkimming,
    explosions_cause_fall_damage_is_triggered_by_explosions: bool,
    explosions_push_all_objects: TriState,
    undetermined_crates: TriState,
    undetermined_fuses: TriState,
    pause_timer_while_firing: bool,
    loss_of_control_doesnt_end_turn: bool,
    weapon_use_doesnt_end_turn: bool,
    above_option_doesnt_block_weapons: bool,
    pneumatic_drill_imparts_velocity: TriState,
    girder_assist_radius: TriState,
    petrol_turn_decay: i16,
    petrol_touch_decay: u8,
    maximum_flamelet_count: u16,
    maximum_projectile_speed: FixedPoint,
    maximum_rope_speed: FixedPoint,
    maximum_jetpack_speed: FixedPoint,
    game_engine_speed: FixedPoint,
    indian_rope_glitch: TriState,
    herd_doubling_glitch: TriState,
    jetpack_bungee_glitch: TriState,
    angle_cheat_glitch: TriState,
    glide_glitch: TriState,
    skipwalking: Skipwalking,
    // block_roofing: BlockRoofing,
    // floating_weapon_glitch: bool,
    // rubberworm: RubberwormOptions,
    // terrain_overlap_phasing_glitch: TriState,
    // fractional_round_timer: bool,
    // automatic_end_of_turn_retreat: bool,
    // health_crates_cure_poison: HealthCratesCurePoison,
    // rubberworm_kaos_mod: u8, // todo
    // sheep_heavens_gate: u8,
    // conserve_instant_utilities: bool,
    // expedite_instant_utilities: bool,
    // double_time_stack_limit: u8,
}

#[derive(Debug, Serialize, Deserialize, DekuRead, DekuWrite)]
struct Scheme {
    header: Header,
    options: Options,
    weapons: Weapons,
    super_weapons: SuperWeapons,
    extended_options: ExtendedOptions,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let (_, scheme) = Scheme::from_bytes((&buffer, 0)).unwrap();

    // println!("{:#?}", scheme);

    // let toml = toml::to_string(&scheme).unwrap();
    // let json = serde_json::to_string_pretty(&scheme).unwrap();
    // let yaml = serde_yaml::to_string(&scheme).unwrap();

    let mut serializer =
        ron::ser::Serializer::new(std::io::stdout(), Some(PrettyConfig::new()), true).unwrap();
    scheme
        .serialize(&mut serializer)
        .expect("Failed to serialize");

    // println!("{}", ron);

    Ok(())
}
