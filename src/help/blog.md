# Raspberry Pi Rover Progress Log

## Phase 1: Initial Setup and Motor Driver Debugging

I did not do a great job documenting the first few steps of the project, so here is a quick recap.

First, I set up the Raspberry Pi so I could SSH into it from my laptop/PC. My goal was to use the Pi headless, without needing a monitor, keyboard, or mouse directly connected to it.

After that, I ran a simple LED test to make sure the Raspberry Pi GPIO pins were working correctly. Once that worked, I connected two motors to a motor driver and a mini breadboard, then connected everything directly to the Pi. The motors worked, I was happy, and I went to sleep.

Later, while watching a YouTube tutorial, I noticed that the person had connected all of the Raspberry Pi GPIO pins to a breadboard using an extension cable. I decided to buy a similar kit to make the wiring cleaner and easier to manage.

When I tried to set up the motors again using the new kit, they did not work. I spent hours rewiring everything and troubleshooting the circuit. Eventually, I found the issue: the pins on the motor driver were not all making proper contact because I had not soldered them, and they were not positioned correctly.

After fixing the connection issue, the motors worked again.

We were so back.

---

## Phase 2: Speed Control

**Objective:** Update the rover so the motors can run at specific speeds instead of only being fully on or fully off.

### 6/26 — 05:00

Sending a specific speed value can result in an error, so I started going through the code and updating the functions I had already created.

This was a bit annoying because I wish I had designed the functions with speed control in mind from the beginning, but it was a good learning experience. We live and learn.

### 6/26 — 06:30

Finished making the changes and confirmed that everything worked.

The motors can now move at specific speeds between `0.0` and `1.0`.

I am considering changing the logic for how steering left and right works. Currently, to turn left, only the right wheels move, and to turn right, only the left wheels move. This works as a tank-style turn, but I am concerned that the inactive wheels may drag against the ground.

I may change the logic so the slower side still moves at 50% speed instead of stopping completely.

Stay tuned.

### 6/26 — 07:45

Stay tuned update: I did it.

The steering logic now allows the opposite side wheels to move at 50% speed while turning instead of stopping completely. I also refactored the code and it looks cute and demure (idk I heard someone say that once).

---

## Next Step: Keyboard Control

The next goal is to control the rover using the keyboard.

Planned controls:

| Key | Action           |
| --- | ---------------- |
| `W` | Move forward     |
| `A` | Steer left       |
| `S` | Move backward    |
| `D` | Steer right      |
| `Q` | Tank turn left   |
| `E` | Tank turn right  |
| `M` | Increase speed   |
| `N` | Decrease speed   |
| `B` | Brake            |
| `X` | Exit the program |

This will make testing the rover much easier because I will be able to control it interactively instead of hardcoding movement sequences.

### 6/26 — 10:45
Guess what? It works. I can control the rover from the keyboard and it drives exactly how I want it.

Next step is to figure out how to wire the sensors. 

I am a bit worried about space and might end up attaching only one of the sensors. We'll see.

### 6/30 — 10:00
Took a long break. Was busy with work and other stuff, but we are back.
I wish I color coded the wires. Todays objective is to figure out the wiring for the four sensors.
Should just be copy paste for all four (I hope).
I am thinking of putting each one on its own mini breadboard.

### 6/30 — 11:00
Yeah still haven't gotten it to work. I am awful at wiring.

Dude. I didn't have the board plugged in this whole time. Jeez man. I stink lol.

### 6/30 — 11:40
Okay. Sensors are all wired. One was connected and tested. Going to do the others after this meeting.
If they all work once connected. I need to figure out how to sit them all on the rover. After that I will worry about the code.