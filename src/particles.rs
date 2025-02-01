use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub fn setup_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands
) {
    setup_player_dash_effect(&mut effects, &mut commands);
    setup_swarmling_death_effect(&mut effects, &mut commands);
}

#[derive(Resource)]
pub struct PlayerDashEffect {
    pub handle: Handle<EffectAsset>
}

fn setup_player_dash_effect(
    mut effects: &mut ResMut<Assets<EffectAsset>>,
    mut commands: &mut Commands
) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 1., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    //There are attributes, modifiers and properties
    //attribute: quantity stored per particle for all particles
    //properties: named variable stored per effect (cannot vary between particles)

    let writer = ExprWriter::new();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(3.).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Initialize the particle size to a specific value
    let init_size = SetAttributeModifier::new(
        Attribute::SIZE,
        writer.lit(2.).expr() // Set the size to 2.0 (default is usually 1.0)
    );

    info!("{:?}", Attribute::all());

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(8.).expr(),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = writer.lit(3.).expr(); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = writer.lit(Vec3::new(0., 0., 0.)).expr();
    let update_accel = AccelModifier::new(accel);

    // Create a new expression module from the writer
    let mut module = writer.finish();

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        Spawner::once(5.0.into(), false),
        // Move the expression module into the asset
        module
    )
        .with_name("DashParticleEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_size)
        .update(update_accel)
        // Render the particles with a color gradient over their
        // lifetime. This maps the gradient key 0 to the particle spawn
        // time, and the gradient key 1 to the particle death (10s).
        .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands.insert_resource(PlayerDashEffect {
        handle: effect_handle,
    });
}

#[derive(Resource)]
pub struct SwarmlingDeathEffect {
    pub handle: Handle<EffectAsset>
}

fn setup_swarmling_death_effect(
    mut effects: &mut ResMut<Assets<EffectAsset>>,
    mut commands: &mut Commands
) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 1., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    //There are attributes, modifiers and properties
    //attribute: quantity stored per particle for all particles
    //properties: named variable stored per effect (cannot vary between particles)

    let writer = ExprWriter::new();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(3.).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Initialize the particle size to a specific value
    let init_size = SetAttributeModifier::new(
        Attribute::SIZE,
        writer.lit(2.).expr() // Set the size to 2.0 (default is usually 1.0)
    );

    info!("{:?}", Attribute::all());

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(8.).expr(),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = writer.lit(3.).expr(); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = writer.lit(Vec3::new(0., 0., 0.)).expr();
    let update_accel = AccelModifier::new(accel);

    // Create a new expression module from the writer
    let mut module = writer.finish();

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        Spawner::once(5.0.into(), false),
        // Move the expression module into the asset
        module
    )
        .with_name("DashParticleEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_size)
        .update(update_accel)
        // Render the particles with a color gradient over their
        // lifetime. This maps the gradient key 0 to the particle spawn
        // time, and the gradient key 1 to the particle death (10s).
        .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands.insert_resource(SwarmlingDeathEffect {
        handle: effect_handle,
    });
}