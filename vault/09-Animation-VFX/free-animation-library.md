---
description: Free animations for Roblox games: the 17 official animation packs' asset IDs, default emotes, and free tools for custom moves.
type: reference
status: verified
last_checked: 2026-11-14
keywords: animations, free animations, animation pack, walk, run, jump, idle, ninja, knight, zombie, robot, cartoony, emotes, animate script, mixamo
sources: Roblox Creator Docs animation/using.md (Roblox/creator-docs repo, read 2026-11-14)
verify: Pack IDs copied from Roblox's own docs table on 2026-11-14 (17 packs). Roblox's docs show using them to replace a character's default animations.
tags: [animation-vfx, reference]
---
# Free animation library

## Do this
- To give the whole game a movement style, swap the IDs in the character's Animate script on spawn (Roblox docs: `animateScript.run.RunAnim.AnimationId`, `walk.WalkAnim`, `jump.JumpAnim`, `idle.Animation1` and `Animation2`, `fall.FallAnim`, `swim.Swim`, `swimidle.SwimIdle`, `climb.ClimbAnim`). Use `rbxassetid://<id>`.
- A shop can sell a movement style as a cosmetic (for example "Ninja run") by swapping these IDs for players who own it.
- Default emotes every player has: dance, point, cheer (chat `/e cheer`, or the emotes menu). Extra catalog emotes are bought by players on the Marketplace; games cannot give them away.
- Custom moves (attacks, building, celebrating): Roblox Animation Editor or Moon Animator 2 (free), Mixamo clips (free account; test that they fit Roblox rigs), or the Modeler agent in Blender. See [[09-Animation-VFX/character-movement-animations]] and [[09-Animation-VFX/combat-animation]].

## The 17 official animation packs (asset IDs from Roblox's docs)
| Pack | Idle | Walk | Run | Jump | Fall | Climb | Swim | Swim idle |
|---|---|---|---|---|---|---|---|---|
| Astronaut | 891621366, 891633237, 1047759695 | 891636393 | 891636393 | 891627522 | 891617961 | 891609353 | 891639666 | 891663592 |
| Bubbly | 910004836, 910009958, 1018536639 | 910034870 | 910025107 | 910016857 | 910001910 | 909997997 | 910028158 | 910030921 |
| Cartoony | 742637544, 742638445, 885477856 | 742640026 | 742638842 | 742637942 | 742637151 | 742636889 | 742639220 | 742639812 |
| Elder | 845397899, 845400520, 901160519 | 845403856 | 845386501 | 845398858 | 845396048 | 845392038 | 845401742 | 845403127 |
| Knight | 657595757, 657568135, 885499184 | 657552124 | 657564596 | 658409194 | 657600338 | 658360781 | 657560551 | 657557095 |
| Levitation | 616006778, 616008087, 886862142 | 616013216 | 616010382 | 616008936 | 616005863 | 616003713 | 616011509 | 616012453 |
| Mage | 707742142, 707855907, 885508740 | 707897309 | 707861613 | 707853694 | 707829716 | 707826056 | 707876443 | 707894699 |
| Ninja | 656117400, 656118341, 886742569 | 656121766 | 656118852 | 656117878 | 656115606 | 656114359 | 656119721 | 656121397 |
| Pirate | 750781874, 750782770, 885515365 | 750785693 | 750783738 | 750782230 | 750780242 | 750779899 | 750784579 | 750785176 |
| Robot | 616088211, 616089559, 885531463 | 616095330 | 616091570 | 616090535 | 616087089 | 616086039 | 616092998 | 616094091 |
| Rthro | 2510197257, 2510196951, 3711062489 | 2510202577 | 2510198475 | 2510197830 | 2510195892 | 2510192778 | 2510199791 | 2510201162 |
| Stylish | 616136790, 616138447, 886888594 | 616146177 | 616140816 | 616139451 | 616134815 | 616133594 | 616143378 | 616144772 |
| Superhero | 616111295, 616113536, 885535855 | 616122287 | 616117076 | 616115533 | 616108001 | 616104706 | 616119360 | 616120861 |
| Toy | 782841498, 782845736, 980952228 | 782843345 | 782842708 | 782847020 | 782846423 | 782843869 | 782844582 | 782845186 |
| Vampire | 1083445855, 1083450166, 1088037547 | 1083473930 | 1083462077 | 1083455352 | 1083443587 | 1083439238 | 1083464683 | 1083467779 |
| Werewolf | 1083195517, 1083214717, 1099492820 | 1083178339 | 1083216690 | 1083218792 | 1083189019 | 1083182000 | 1083222527 | 1083225406 |
| Zombie | 616158929, 616160636, 885545458 | 616168032 | 616163682 | 616161997 | 616157476 | 616156119 | 616165109 | 616166655 |

Note: Roblox's table lists the same ID for the Astronaut pack's Run and Walk; that is how the docs show it.

## Matching packs to game feel
| Game feel | Try |
|---|---|
| Cozy, cute | Bubbly, Cartoony, Toy |
| Action, fast | Ninja, Superhero, Stylish |
| Fantasy | Knight, Mage, Levitation |
| Spooky | Zombie, Vampire, Werewolf |
| Sci-fi | Robot, Astronaut |
| Grounded, realistic | Rthro |
