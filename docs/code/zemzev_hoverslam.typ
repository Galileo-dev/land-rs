```python
import math

def perform_hoverslam(ship_altitude, vertical_speed, available_thrust, ship_mass, body_gravity, current_pos, landing_site):
    # Stopping distance / Time-to-go
    def calculate_stop_distance(vert_speed, decel):
        return (vert_speed ** 2) / (2 * decel)

    # Ideal throttle so we reach 0 veclocity at 0 distance
    def calculate_ideal_throttle(available_thrust, engine_count, ship_mass, body_gravity, vertical_speed, true_radar):
        decel = (available_thrust / engine_count / ship_mass) - body_gravity
        stop_distance = calculate_stop_distance(vertical_speed, decel)
        return stop_distance / true_radar

    # error between current and target pos
    def calculate_error_vector(current_pos, landing_site):
        return (landing_site[0] - current_pos[0], landing_site[1] - current_pos[1])

    def adjust_angle_of_attack(true_radar):
        if true_radar < 7000:
            return 5
        elif true_radar < 12000:
            return 7.5
        else:
            return 10

    # Main calculations
    max_decel = (available_thrust / ship_mass) - body_gravity
    stop_distance = calculate_stop_distance(vertical_speed, max_decel)
    true_radar = ship_altitude
    aoa = adjust_angle_of_attack(true_radar)

    # Guidance loop for pos before landing burn
    while true_radar > stop_distance * 3:
        error_vector = calculate_error_vector(current_pos, landing_site)
        steering_correction = tuple(e * 0.2 for e in error_vector)
        current_pos = tuple(cp + sc for cp, sc in zip(current_pos, steering_correction))
        true_radar -= vertical_speed
        aoa = adjust_angle_of_attack(true_radar)

    engine_count = 3
    ideal_throttle_three_engines = calculate_ideal_throttle(
        available_thrust, engine_count, ship_mass, body_gravity, vertical_speed, true_radar)

    engine_active = engine_count
    if ideal_throttle_three_engines < 0.7:
        engine_active = 1

    throttle = 1 if engine_active == engine_count else max(0.3, ideal_throttle_three_engines)

    # Update vertical speed and altitude during final descent
    def update_velocity(vertical_speed, throttle, max_decel):
        return vertical_speed + throttle * max_decel

    while vertical_speed < -0.01:
        vertical_speed = update_velocity(vertical_speed, throttle, max_decel)
        true_radar -= vertical_speed
        ideal_throttle_three_engines = calculate_ideal_throttle(
            available_thrust, engine_active, ship_mass, body_gravity, vertical_speed, true_radar)
        throttle = ideal_throttle_three_engines

    return "Hoverslam Completed"
```
