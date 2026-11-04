// Outpost 2: Divided Destiny Novella by J. Steven York
// Extracted from original story.vol archive

#[derive(Debug, Clone)]
pub struct NovellaChapter {
    pub id: &'static str,
    pub title: &'static str,
    pub is_eden: bool,
    pub chapter_num: usize,
    pub text: &'static str,
}

pub static NOVELLA_CHAPTERS: &[NovellaChapter] = &[
    NovellaChapter {
        id: "s_e01.rtf",
        title: r###"Chapter 1: Conspiracy"###,
        is_eden: true,
        chapter_num: 1,
        text: r###"Both wheels of the scooter left the ground as Axen sailed over the lip of the down ramp and into the tunnel that connected the Hot Lab with the rest of Eden colony. He touched down a third of the way along the ramp, the tires landing with a satisfying ch

irp that echoed off the metal-lined walls, motors whining in protest. He was breaking half a dozen safety regulations, driving like a teenager a third his biological age of forty-five.

He didn't care. First of all, he was mad, damn it, and it felt good to break a few regs, and second of all, nobody was around to complain. The tunnels were almost deserted. Every adult who wasn't engaged in a service vital to the maintenance of the col

ony was in Nguyen's "town meeting," the meeting Axen Moon had just walked out on.

He flew past a huge set of pressure doors, startling a workman who was inspecting the utility conduits that lined the wall like rows of fat sausages. He squealed the scooter

around a right angle turn and up the tunnel toward his residence unit. He slowed slightly as he passed a group of children and their teacher-bot strolling down the ramp from the Nursery, then twisted the throttle hard over to scream up the last bit of tu

nnel. He hit the base of the ramp with a bump, started braking halfway up, and slid to a stop just short of the open airlock doors.

He nosed the scooter into the charging station next to a row of identical vehicles, and plucked his keycard, with its hacke

d safety overrides, from the slot in the handlebar. He palmed the card and glanced out through the tran-station's observation port. The sun was setting outside, exaggerating the hard reddish tones of New Terra's landscape. The buildings of Eden spread

out before him like a cluster of silver toadstools. In the distance he could see the farthest of the lab structures, nicknamed the "Hot Lab," where the meeting would still be going on.

f a fool until now. He rubbed the keycard between his fingers. The main lock into the Hot Lab was the one door in Eden it wouldn't open. Axen Moon wasn't a man who liked anything closed to him. He wasn't a man who was used to it.

He strolled through the safety lock, its open doors ready to spring shut at any sign of an emergency. New Terra's thin atmosphere would kill a human in less than two minutes. It was something you were either eternally aware of, or you were dead. The co

mmon area, with its lo

unging chairs, planter islands, and multitainment consoles, was deserted, as he'd hoped. He was about to break a law much more severe than a scooter speed-limit, the one law that carried a death penalty, and he didn't want any potential witnesses around.

He waved the key in front of the door to his private quarters and stepped quickly inside.

"Good evening, Axen." The voice was cool, female, with the slight accent that all Savant series computers shared. The computer itself, a glossy black cube a littl

e less than a meter on a side, was recessed into a console on the inside corner of the room.

A window on the computer's otherwise featureless surface displayed a moving, transparent gear-works, like a clock made of glass. This was Kraft's identity icon, its face in a way, as familiar to Axen as his own. It was as much a roommate as an appliance

. The Savants were the most sophisticated computers ever made, almost human in many ways, undoubtedly superior in others.

"Good evening, Kraft. Verify security?"

"We are secure, code-word 'collusion.' "

He nodded, the gesture doubtless detected by one of the Savant's many eyes hidden around the room. He sat down on one of the room's two chairs. He had what were considered luxury quarters by Eden standards, but the room was only two meters by three, and

would have been smaller yet if he hadn't been allowed a little extra space for Savant Kraft.

Except for a few of the most advanced researchers in the labs, only the handful of surviving Elders, such as himself,

were allowed their own Savants. He and Kraft had been together since he'd emerged from cold sleep on the starship ten years out from New Terra. He'd been just a child then, with only dim memories of Earth, open skies, and plants that didn't grow in a h

ydroponics vat. He sometimes wished those memories meant more to him. Eden was home now, for better or worse. Earth was dead and nearly forgotten.

He sighed. "Kraft, open a stealth backchannel to Savant Frost. I need to talk with Emma." There. He'd

done it, initiated a clandestine communication with the rogue Plymouth colony, an act of treason that could get him kicked out of the nearest airlock.

"One moment. Emma is in her quarters. Frost confirms that she is secure. Opening voice."

"Open visual."

"Confirm?" Kraft sounded incredulous, if that were possible for a computer. Savants weren't supposed to have emotions, but there were those who had their doubts. Certainly Kraft had the justification. Visual communications would take a hundred tim

es the bandwidth of voice-only, with a correspondingly increased chance that their link, bootlegged on the subcarrier of a satellite control signal, would be detected.

"Confirmed. I want picture."

One face of the Savant's cube brightened into a display. Routing it through the internal network to the room's EnterCom screen would have been less secure. Emma's thin, high-cheekboned face turned toward him. Her blond hair, streaked with gray, was pil

ed on top of her head and held in place with a couple of writing styluses poked into the bun. It had been years since he'd seen her face. Sometimes he still missed her. This was one of those times.

She looked into the camera, her eyes wide with surprise. "Axen, are you crazy?"

"You need to see my face, Emma, to know I'm serious. This is worth the risk."

Her brow wrinkled with concern, and she sat down. Behind her he could see her quarters, if anything, smaller than his own, and as always, a disorderly heap of clothing, rock samples, and scientific equipm

ent. "What's wrong? You're not the joking kind, Axen."

"It's Nguyen. I told you he'd slammed the lid down on one of the labs three years ago. We'd assumed he was working on biotech for terraforming. He's resisted all my efforts, legitimate and clandestine, to get inside. His scientists are all handpicked,

and not about to talk."

"And?..."

"Today he threw the lid open, held a town meeting to show off his secrets. He was working on terraforming, but that's only a small part of it. He's also been mining the encrypted data files from the starship, the ones on military technology."

He heard an almost imperceptible gasp from Emma. The Founders on Earth had been reluctant to throw away any science, but they'd also hoped that the new world could avoid some of Earth's worst mistakes. Thus, certain information had been encrypted with t

he desire that it remain that way until the new civilization was ready for it. "How bad is it?"

"He has high-energy weapons, Lasers of some kind, I think. He was hardly forthcoming with technical details, though he was all too happy to blast a hole through a piece of hull-metal as a demonstration. There may already be fixed installations on the ne

w security posts, and he's working on adapting them to a turret on one of the mobile units. Maybe he has them operational already. I wouldn't put it past Nguyen to feed us misinformation."

She smiled slightly. "As though you and I don't know a thing or two about misinformation. Where did we go wrong, Axen?"

He leaned his head down and rubbed his brow, unable to face her. This had been his idea initially. "You saw the computer projections. All the Elders did. Two independent colonies had a much greater probability of survival than one."

"We could have told the people..."

"Not and have the colonies be truly independent. Creating a political rift seemed like the best way."

She nodded. "And now you see where it's brought us?"

He sighed and leaned back in his chair. "You can see for yourself the different paths Eden and Plymouth have followed. The split might have happened anyway, given enough time. It's human nature to cluster into like groups."

"It's human nature for groups to go to war, too. We should have seen it coming."

"There isn't a war yet, Emma. Plymouth doesn't even have weapons."

"They will, when they find out. They'll be forced into it."

"How will they find out?"

She frowned, and a bit of the old fire flashed in her eyes. "I'll tell them, Axen if I have to. Of course there are probably less direct ways. There always are."

He didn't agree, but he couldn't argue. He'd do the same thing if he were in her place. Why else had he contacted her?

"Besides," she continued, "if they start on terraforming without our consent, there'd have to be a reaction of some kind. It goes against the principles that Plymouth was founded on, of living in harmony with New Terra rather than trying to make it into

some kind of ersatz Earth..."

He cleared his throat and squirmed in his seat. "Emma, from what I saw this afternoon, I'm pretty sure they've already started, some kind of atmosphere building microorganism injected into the bedrock. Just test wells so far, but..."

"What! What the frag do you people think you're doing?"

He signed. "They hardly consulted me, Emma. You know that I think any such drastic step has to be by consensus."

"Which is a little hard to do when the colonies aren't even talking to each other."

He

could feel old wounds opening, feel them falling into the pit of their own differences, as powerful as the attraction that had once brought them together. "It was your people who shut down the communications satellite. We don't have the technology to tal

k even if we wanted to."

"It was an accident, Axen. The Council just wanted to make a dramatic gesture. They had no idea that the satellite couldn't be turned back on. Besides there's still our backchannel through the weather satellite telemetry links. It's enough to open som

e dialogue between our leaders."

"It's too late for that. I've been trying to talk sense into Nguyen for years, face to face. What chance does some voice out of a box have?"

Her look turned deadly serious. "Then it might be time to take Nguyen out of the picture, Axen."

He felt the hairs stand up on the back of his neck. Emma had a ruthless side that he sometimes forgot. "What are you suggesting?"

"Doing what we've always done, Axen, what's necessary for the survival of the human race."

He shook his head. "No, I can't do that, Emma."

"We're the last of the Elders, Axen, the last colonists born on old Earth. It's a dangerous world, even as protected as we are. We can't risk one of us dying with the Program out of control."

He was considering wha

t she said, which is why he happened to be looking out the window as the fireball erupted. The sound came a fraction of a second later, mostly as a rumble in the floor, the air too thin to conduct much noise. Somehow, without realizing it, he was on his

feet and standing in front of the window. "Maker's name..."

"What," pleaded Emma's voice, "I've lost visual. Axen!"

"Malfunction," said Frost, "major malfunction. I have lost contact with all computers in Lab Structure Four. Twenty seconds prior to this, a major cascade failure moved through all the structures' electronic systems."

"Emma, the Hot Lab exploded. I can see flames, so there's a major oxygen leak. It looks like the whole pressure vessel must be ripped open. Frag, half the town was in there!"

The silence suddenly stuck him. "The alarms! Kraft, where are the alarms?"

"All automatic alarms attached to Lab Four have been disabled, per Chairman Nguyen's orders."

"Sound disaster alarms. Dispatch disaster response teams. Use my override codes." Even as he was finishing the sentence, he could hear the klaxons sounding from every intercom speaker.

"Axen!" Emma called. "What's happening? I can't hear you. The signal's breaking up."

Axen ignored her so he could listen to Kraft.

"Disaster teams will be unable to respond," said Kraft. "Emergency tunnel bulkheads and airlocks were closed before the explosion."

"Open them."

"I do not have those override codes, and the doors have been locked."

"By whom?"

"The doors are locked from inside."

"Axen, what?..." Emma's voice was cut off.

"We have lost the link to Plymouth. There is an incoming communication. It's origin is the Lab Four east safety airlock."

It was the same lock Axen had walked through only ten minutes earlier. Perhaps someone had gotten inside and sealed it before the building had completely depressurized. "On screen."

The big EnterCom screen on the far wall came to life. He recognized Lil Komos, one of Nguyen's scientists, but one of those most friendly to Axen's viewpoints. There were several times he had thought she might agree to be a mole inside Nguyen's operatio

n, but it had never happened. He could tell before she spoke that she was terrified, and something else. She didn't look well. "Elder, I don't have much time. I'm setting this message to record and repeat. Something went wrong with the test well. I

told them the organisms we were using were too dangerous."

"Lil, are you...? "

She continued, either unable to hear him, or unwilling to stop talking. "It grows too fast, but Nguyen wasn't willing to wait. 'An atmosphere in our lifetime,' he said." She shook her head as though to clear it. She brushed her hair out of her face, a

nd he could see that she was trembling. "Not enough time to explain. Attacks organics, even protein units in boptronic computers, even the plastics in our environmental seals."

In the background, he noticed for the first time a slight hissing noise.

"Evacuate, now

. Get everyone you can in the Evac Transports. Salvage what you can, but avoid the affected areas, and don't let anyone out. Get away and don't come back." She coughed. Her face was white and waxy looking, red veins clearly standing out in her cheeks

. "Don't try to rescue us, or investigate. Get out while you can. Only you have the influence to make it happen, Elder, you and Nguyen, and Nguyen is dead. We're a

ll dead. Don't..." Her eyes went wide. Her hands came up as though to cover her face, then stopped halfway, shaking.

She fell back against the far wall of the airlock, her body shuddering convulsively. The camera lingered on her for almost a minute before there was a click, and the message repeated. Axen cut off the picture with a gesture before he could hear her voic

e again.

He leaned against a chair to steady himself. He had to do what she'd said. There was no choice. He'd seen what was happening to her before the message cut off.

She was melting."###,
    },
    NovellaChapter {
        id: "s_e02.rtf",
        title: r###"Chapter 2: Resettlement"###,
        is_eden: true,
        chapter_num: 2,
        text: r###"Brook Panati was attracted to power like an iron filing to a lodestone, and uncommitted power attracted him even more. Young as he was, he was an ambitious man, and while the chaos that had nearly destroyed Eden had caused him grief and hardship, it had

also created opportunities for him.

Before the disaster, he was only a student, working part-time as an engineer's assistant, and third in line for an apprenticeship. Now, with skilled bodies in such short supply, he had a position of authority, a title ("Operations Foreman," necessarily v

ague to cover the number of job holes he had to help fill), and a small power domain of his own.

He enjoyed all this, but he wasn't satisfied with it. He'd seen the shifting of power that had resulted with the emergency relocation of the colony, and the d

eath of the Council and Chairman Nguyen. There had been a mad scramble to grab the seats of power and reestablish order. Someone else might simply have described it as an attempt to restore organization and prevent panic, but to his eyes it was somethin

g else entirely: a game. A game he wanted very badly to play.

For now though, he was stuck on the sidelines, more an interested observer than a participant. So it was in watching the struggle for power and authority that his attention was drawn to Axen Moon.

Moon was something of a legendary figure, the last of Eden's Elders, the last alive to have walked on the soil of old Earth. To some that made him an almost God-like personage, his opinions sought, his endorsements valued. Certainly he could have had an

y position of authority he wanted, and yet he remained aloof, both personally and politically.

Moon's Elder status didn't impress Brook, not for its own sake at least. To him, Moon was just an old man, by Eden standards anyway, who had those around

him cowed. But Brook was fascinated by the power that status represented, and how Moon's aloof nature only added to his authority. He had seen that while titles and authority gave one power, they also came with restrictions and limitations. Whatever p

ower Moon possessed, he wielded it as a free agent, answerable to no one Brook could see.

But while he might not be answerable, it was just possible that he might be influenced in some way, perhaps through an exchange of favors. Everyone needed or wanted

something in the best of times, and now even more so. If Axen Moon needed something, Brook was determined to find out about it.

Thus, he started making an effort to stay close to Moon, which wasn't difficult, considering the horribly crowded conditions of the recently relocated colony. Even the new Council members lacked private quarters. Hammocks were slung in workplaces, bedro

lls put down end to end along both sides of the few tunnels. Technicians napped in the Command Center between double shifts.

Moon was no exception. His importance granted him an actual bunk in a fairly low-traffic hallway off the Agridome, but even there he had no privacy to speak of, only a curtain that could be pulled around the bunk.

What Brook lacked in importance, he made up for in cunning. He began trading small favors for larger ones, a maintenance schedule moved up for someone's convenience, a reallocation of the material resources under his control, trading a shift with a co-wo

rker here, making a timely delivery there. It took time and determination, but Brook soon had his berth relocated to the corridor junction only a few yards from Moon's.

In fact, the bed was harder than his old one, and only a little quieter, but it was the location he was interested in. From there, he would be able to determine what Axen Moon most wanted, and how he would be able to provide it to the Elder.

He began making observations as soon as he moved to the new location. He set his wrist-comp to wake him early, but rather than getting up

, he remained under his blanket and waited for Moon to emerge. When Moon did, Brook would follow him, discreetly, and only as his own schedule allowed. It took him several days, but something of a routine started to appear.

Each morning began with an hour of exercise in the Agridome: first, vigorous calisthenics, some kind of martial art Brook didn't recognize, then jogging around the dome perimeter. If Brook had any illusions that age made the Elder weak, they were dashed.

He followed this with a show

er. Moon never seemed to lack for ration points of any kind. Then he made his rounds of the colony. The purpose of these rounds wasn't immediately clear. Mostly he just wound his way through the crowded corridors and tunnels, watching everyone, but nev

er making eye contact, rarely talking with anyone.

His occasional conversations took place in the shadows, as close to privacy as could be arranged in the current circumstances. In these conversations, things would occasionally change hands, probably the

source of Moon's ration points, and often data-slips and keycards were involved. Information was obviously a commodity that Moon valued highly. But Moon generally came away from these meetings looking unsatisfied. Sometimes, heated words were exchanged

.

These occasions made Brook smile. They indicated a need, an unscratched itch that could be exploited. He just had to identify the source of that irritation.

Beyond this, Moon kept no regular schedule. Though he had no official job, he did his share of colony work, demonstrating skills in basic engineering, several fields of science, boptronics in general, and computers especially.

It wasn't clear how these work assignments came to him, but Brook noticed that they always allowed him to work alone. This isn't a man who will be easy to get close to.

One day, as he sat eating in the hallway outside the operations center, he was surprised to see Moon stroll up, making a conscious effort not to look conspicuous. Anyone not watching Moon as Brook was woul

dn't have noticed a thing. But he sauntered up to the closed door into the building, looking at it with a curious longing in his eyes, as though he wanted to step inside, and couldn't. Then, just that quickly, the expression glazed over, and Moon turned

and walked away.

There's something in there he wants badly. I have to find out what it is.

He returned to his shift distracted and deep in thought. That evening he contrived to run into an old school chum named Della, who was now working in the CC. Wh

en she seemed little interested in engaging in conversation, he told her about the small jug of beer he had stashed in his locker, the equivalent of a month's ration for workers like them, and invited her to share it with him. She immediately agreed.

It wasn't flattering to be upstaged by a few pints of ale, but Brook knew the dangers of false pride. He had no interest in this woman beyond information anyway.

Brook's work included coordinating building setup with colony operations, and so he had access

to the Structure Factory. He let them in with his keycard and they wound their way up narrow stairs and catwalks to a seldom-used control balcony. It wasn't a romantic setting, sitting on a salvaged vehicle seat that some workers had dragged up there f

or their breaks, smelling adhesives and hot metal, listening to the whirring of robots and the crackles of welding in the assembly bay below.

Romance wasn't what either of them had in mind though, and it was as private as he could manage. The occasional wo

rker who wandered through paid them little attention, and the noise kept them from being overheard. The view was entertaining as well, as the robots performed their intricate dance among welding sparks and showers of flying plastic chips.

Brook poured most of the beer out into two large cups and handed one to Della. She sipped it, and after swishing it around for a while, smiled.

"A guy I know in the Agridome Jix makes it from stuff he cleans out of the grain processors after a run. It's not on the ration inventory, and every once in a while he trades some for a favor."

"It's good. I'm surprised you'd share it." She made eye contact with him, and he caught a flash of suspicion there.

"I'm not a big drinker. Mainly I use it for barter. But I was thinking the other day how few of my old schoolmates were left." The suspicion in her eyes turned immediately to sadness. He'd touched something there. Clearly she'd remained closer to her

classmates than he had, and many of them had been lost in the disaster. "I decided it was time to look some of you up and have a chat."

From there, she fell naturally into conversation. They talked for hours, long after the beer was gone, about old friends and new, adventures large and small, with Brook carefully avoiding the one subject he wanted most to ask about.

Finally, they came back to talking about work and Brook saw his opportunity. "I saw Elder Moon outside the CC today. Do you see much of him there?"

Della wrinkled her nose and laughed. "Sometimes. Quite a bit really. He always has some legitimate business, a repair to do, or a program to update, or a transmitter to calibrate, but I think he really comes just to visit his computer."

"Computer?"

"His personal Savant. All the Elders had them from their days on the starship, and they were allowed to keep them out of respect, I guess. But after the disaster, Savants were in short supply, and they appropriated his for the CC. He made a huge stink

about it with the Council, but there wasn't much he could say, really."

"He comes to visit it?"

"He touches it. Talks with it sometimes, in whispers, when he thinks nobody is listening, which isn't often, as busy as the CC is.

"Me, I'd just as soon he took Kraft back

that's its name, Kraft. It works well enough, but it's a creepy old thing, one of the first Savants made on the starship, and it's gotten really eccentric." She giggled. "It talks down to us. I think it think

s it's smarter than we are. Thing is, I'm not sure it isn't right."

That's it. The key to Axen Moon is that computer. Now I just have to find a way to get them back together.

Brook took the long way back to his berth and noticed that Moon wasn't in bed yet. Unusual, he wasn't generally a night owl. Brook looked longingly at his bunk and decided it would have to wait.

He wandered the tunnels of the colony, hoping to spot Moon. There was a lot of ground to cover, but knowing what he did now, he chose to focus his attention on the area of the CC. At first he had no luck, then h

e decided to try a narrow maintenance corridor off one of the connecting tunnels. The corridor was barely a meter wide, and he had to step carefully over a couple of people in sleeping bags.

Deeper in the tunnel, crates of supplies lined one wall and he had to crab sideways to get past. Supply conduits lining the ceiling thumped like the slow beating of some monster's heart. He moved slowly, peering cautiously ahead. Then he spotted Moon w

orking, a service panel pulled away from the wall.

Brook ducked

down behind a crate, hoping he hadn't been seen. He waited there for several minutes, then cautiously peered over the crate. Moon was gone, though the panel remained open, and a boptronic tool kit was spread out on the deck below. Brook moved from his

hiding place to make a closer inspection.

The panel had a security warning label on it and a coded lock, but it hadn't been forced. Inside, armored conduits ran like metal snakes into a junction box with another security lock. Connected to the lock was

a seemingly handmade boptronic device that he didn't recognize. It was a safe guess that at least some of those conduits ran to the Command Center above them.

He was bending for a closer look when a shadow dropped from the bank of conduits suspended above him. Powerful hands grabbed the front of his jumpsuit, lifting him off his feet and slamming his back painfully against the wall next to the panel.

From down the corridor, a sleepy voice yelled for them to keep it down.

As Brook tried to catch his breath, his feet held dangling a few inches off the floor, he realized that even his revised assessment of Moon's physical condition had underestimated him.

He looked into the Elder's face, only a few inches from his own, the features angry but carefully controlled. "All right, Brook Panati, you've been following me. I want to know why. Now ."

Brook had obviously underestimated him in other ways, too.

Brook ignored the question and smiled, moving his eyes toward the open panel. "That's a very dangerous thing you're trying to do there. What are the chances you can tap into the CC's internal computer network without being detected?"

Something in Moon's face changed, as though he were reassessing Brook. "Better than you think."

"But not good. You must want access very badly to take such a risk. There are other ways."

"What do you mean?"

"I can give you access to your computer, without all this risk."

Moon just looked at him silently, and Brook had the feeling that those dark eyes could see right through him. Then he slowly lowered Brook's feet to the floor and released his grip. "We'll talk, but you'd better not try to trick me. Never trick the tri

ckster."

Brook straightened his jumpsuit and smiled for show. It didn't matter how perceptive Moon was, he'd been telling the truth. He fully intended to give him his computer back.

The problem was, Brook didn't have the slightest idea how ."###,
    },
    NovellaChapter {
        id: "s_e03.rtf",
        title: r###"Chapter 3: Breakthrough: Accomplice"###,
        is_eden: true,
        chapter_num: 3,
        text: r###"Axen didn't trust Brook Panati, but he found him to be useful, and for now that was enough. Axen's traditional methods had depended on stealth, making small changes at key points to achieve large long-term effects, and most of all, patience. Eden's curr

ent status disrupted all of that.

Whatever Nguyen had set loose,

it continued to advance unpredictably. The colony had been forced to relocate again, and it appeared it would not be the last time. The Council had made the decision not to tax the colony's resources with a long and dangerous migration. Instead they w

ould move a shorter distance, dig in, build their resources, and hope for a solution. At least from here on out any relocations would be planned and, to the degree possible, orderly.

But that didn't help Axen much. Anything "long-term" was uncertain and

subject to interruption. His method of manipulation and plotting didn't work. It was like trying to build a house of cards during a quake. Young Panati's methods on the other hand were brash, quick, developed on the fly, and Axen had to admit, better s

uited to the world as it stood. As a result, Axen not only used Panati, he watched him, learned from him.

Today, Axen sat in a corner of the crowded common area of the first new residence module constructed since Eden's last relocation, and observed the young man wheeling and dealing on the far side of the room.

Panati's blue eyes sparkled, his smile flashed like a dueling saber, as he cajoled a short-haired blond woman Axen recognized as one of the housing supervisors. They sat on a bench surrounded by adults and children, chattering, crying, working, reading,

playing, mostly pointedly ignoring each other in the pretense that had come to serve them as a substitute for privacy.

The woman was frowning, but although Axen was too far away to hear the conversation, he had the feeling that she was caving in. He had to admit that Panati was as skillful with charm and salesmanship as Axen was with authority and logical persuasion.

Axen's web of contacts, informants, and influences had been shattered by the disaster at Eden. Most of these had been among the older colonists, the first sons and daughters of Axen and his fellow Elders, born in those frantic first few years after landi

ng. They had been conceived in that honeymoon between the euphoria of surviving the voyage, and the horror that the Elders lives were being cut short by hibernation syndrome.

Axen had watched as that first generation, all his closest contemporaries, died around him. Only a handful, including Axen and Emma, were young enough, and had been wakened early enough, to escape the brunt of that terrible sickness. Now he had watched

most of another generation cut down before its time.

It weighed heavily on him. He'd lost too many friends in that first tragedy, and he'd since considered friends a luxury he couldn't afford.

Panati represented another age group, some of them children of Elders, some of the following generation, but socially divided as though by a wall. This was the comfortable generation, which had grown up in an Eden that had become fat, happy, and fearless

. For them, survival had never seemed an uncertainty, want had never been their companion. Until the disaster. Until now.

Axen had to admit that Panati was well connected. In the aftermath of the disaster, evacuatio

n, and relocation, he and his contemporaries had been pushed into positions of control and authority. While some would have become slaves to duty, or inversely, resentful of the burden, Panati both shouldered the load and immediately saw the opportunitie

s created.

He'd maintained every contact available to him, and had used these to make more. Single-handedly he'd created a black market in luxury goods and services, and made it so efficient and useful that the authorities were forced to look the other wa

y. In his own way, Panati provided a service that the struggling colony couldn't do without.

Until his violent first meeting with Axen, those contacts had been used only in the cause of petty racketeering, but Axen had immediately seen more substantial possibilities, even though Panati had failed to deliver on his dubious promise to deliver Kraft

back into his possession. In any case, the disruption of the latest evacuation had given him a reprieve on that offer.

Across the room the conversation seemed

to be ending. The blonde woman glanced at Axen without making eye contact or acknowledging his presence, then stood and made her way out through the safety lock. Panati in turn stood and strolled casually toward Axen. He sat down at the other end of th

e bench and pretended to be studying a form displayed on his ClipCom.

Axen glanced at him without moving his head. "Well?"

Panati smiled slightly. "It's a done deal. I shift some schedules so she gets her next residence module two weeks early, and you'

ve got room thirteen over there," he motioned with his head toward the left wall. "You can move in any time. I'll send a cargo cart down to pick up your stuff."

Axen shook his head slightly. "Not just yet. Did you set up the meeting with your scientist friend?"

"They're very busy getting the new lab up to speed. He's working double shifts..."

"Did you set up the meeting?"

The corners of Panati's mouth twitched down. "Yeah, but it cost me darned near as much trade value as getting you private quarters."

"It doesn't matter. Kraft might be able to tell us something about what went wrong back at the Hot Lab. Failing that, we have to know what the authorities know."

"They've already told us. An advanced nuclear power source ran out of control and flooded the area with hard radiation."

Axen smirked in spite of himself. "You believe that?"

"What else would it be?"

"I don't know, but I know what radiation poisoning looks like, and something else killed those people, something chemical, or biological."

"You weren't there when the accident happened. How could you?..." He looked over, abandoning the pretense of ignoring Axen. "You're keeping things from me again, aren't you?"

"I tell you what you need to know. We talk to your friend, then perhaps we'll both know what happened. When and where?"

"The maintenance tunnel under the CC, in half an hour. Why can't we just meet in your quarters? I thought the whole point of arranging for them was to get privacy."

Axen sighed. "Privacy that I don't want to compromise with visitors. I don't want to draw attention by having a constant stream of strangers going in and out of my room. I've told you again and again that I don't want my associations to be that obvious

."

Panati scratched his nose. "Okay, okay. I was just asking."

"I saw you with Della again last night. That's the third time this month."

Panati frowned and flexed his hands nervously. "We're drinking buddies these days, that's all. She might be useful if we're ever going to get your computer back."

"I know that, but it doesn't pay to get too close."

"I'll be the judge of that. Listen, we have a working relationship here. You just stay the frag out of my personal life. Understood?"

Axen said nothing. He had denied himself companionship for too long to have much sympathy for the younger man. He knew too well the problems such entanglements caused.

His relationship with Emma had worked only because they had been in the conspiracy as equals. It had failed for the same reason.

Panati leaned forward half out of his seat. "I'll meet you down there in twenty. Later." He stood and strolled out of the room as casually as he'd entered it, expertly hiding any anger he might be feeling.

Axen slumped back in his seat and sighed again. He liked working alone, and resented having to depend on this child. Panati's motives were less than clear to Axen, and

that made him uncomfortable. Axen could use him, but he couldn't trust him, and didn't know if he could depend on him. It was the kind of uncertainty that kept Axen awake at nights.

But for Axen, there was no choice whatsoever.

d # # #

The tunnel was narrow, nearly identical to the one where he'd first met Panati. It gave him a sense of d\'e9j\'e0 vu

. He'd even been here before, right after the relocation, to investigate the possibility of contacting Kraft. He could still see the scratches where he'd removed a service panel. Sloppy work, he thought.

Panati had been right, of course, and might have saved Axen from a terrible mistake. His plan, while it might have worked, had been far too risky, born more of desperation than reason. Patience had always been his forte, and in this one instance it migh

t still be the best plan. He glanced at the chrono display on his wrist link. Patience had its limits. Where were they?

A pair of figures slid around the nearest bend of the corridor. One of them was Panati, the other, a shorter man with a round face and skin the color of dark chocolate, had to be the scientist.

Axen stared at Panati. "You're late."

"It's my fault, Elder," said the scientist. He stuck out his hand. "Eldon's the name, Eldon Jensen. You presented me a science award back in the third grade. It's a real honor."

Axen had a vague recollection of the school visit, before he had become the last Elder in Eden and the burden of secrecy had fallen solely on him. Axen suppres

sed an annoyed smile. Evidently it wasn't enough of an honor for Jensen to make the meeting without a substantial bribe.

"So," continued Jensen, "what did you want to know?"

Axen studied the man. He seemed guileless, but one could never be sure. Certainly he had already proven himself greedy enough to be bought. "I trust you understand that absolute discretion is necessary here, Jensen. I don't want this conversation gett

ing back to the Council, and anyone else for that matter."

Jensen smiled. "Of course."

Axen nodded. "Good. As you are certainly aware, Mr. Panati and I are not without... resources. If word of this were to leak we'd be most... upset." The pause was calculated. Sometimes a pause spoke terabytes.

Jensen looked from Axen to Panati nervously. Axen noticed that Panati kept a perfect poker face.

"You can count on me. Completely!"

Axen smiled reassuringly. "Then we can move on. I want to know what went wrong at the Hot Lab. What killed all those people at Eden, and what is forcing us to keep moving?"

Jensen looked nervous. "We... we don't know. Radiation..."

To Axen's surprise, Panati shook his head. "Don't try to scam us, Jensen. We know those people weren't killed by radiation poisoning. Radiation wouldn't continue to spread. That's just a cover story, for something, maybe chemical, more likely biologic

al."

Jensen chewed his lip. "Biological we know that much. Nguyen had them working on a terraforming bug. It was supposed to live in the deep rocks, to crack loose

hydrogen and oxygen bound in the planet's crust. They were into some serious biotech, beyond simple genetics

they were reengineering the internal workings of bacterial cells from the molecules up. They thought they had it too. They were confident enough to begin field tests."

Axen nodded. "Test wells under the lab."

"Those were the start, but they were doing field tests, too, a line of wells running up Yeager Canyon almost to Plymouth, following the natural fault line."

A chill ran down Axen's back. Whatever had happened could potentially have affected Plymouth as well. "Then what went wrong? How did a terraforming bug destroy a lab, and send us scurrying across the landscape like Nomads?"

Jensen looked puzzled. "I assumed you knew, Elder. You were the one who pushed for the first evacuation of the colony. You mean, you didn't even know why?"

Axen frowned. This was exactly the sort of question he didn't want to raise.

Panati jumped in. "We brought you here to answer questions, Jensen, not

to ask them. We've paid for the answers, not you. If the Elder asks questions that he already knows the answers to, it's because he needs to know what you, and in turn, the Council, think the answers are."

Axen raised an eyebrow. Good going, boy.

Jensen took a moment to digest the response. It seemed to go down well. "Without knowing the details of what Nguyen's people were doing, and we don't, we can't say for sure. What we do know is that it has something to do with his terraforming bug, and

it s reproducing and spreading through the deep rock at a phenomenal rate. Along the way it's causing geothermal activity in some way we don't understand yet."

"The tremors we've been feeling lately," said Axen.

"That's just one symptom of a much broader effect. Possibly that's what happened in the Hot Lab, a steam blow-out through the well, or even magma. We just don't know."

Jensen hesitated, as though he thought he might be saying too much. "We've tried to send survey vehicles back into the contaminated area. They all malfunction and stop transmitting shortly after entering. The information we have suggests some breakdow

n of the organic materials in the vehicle: plastics, bio-elements in the boptronics, things like that.

"We think there may be a way to protect a vehicle, at least for a while, and we're working on a way to detect the approach of the bug, the Blight that'

s what we've been calling it. That's about all we know for the moment. What we really need is to get back into Eden."

Jensen stared at Axen, blinking silently. It seemed that there was nothing more forthcoming.

"I want to be kept posted on your progress. If there are any significant developments, contact Mr. Panati at once. If necessary, we'll arrange another meeting." He considered his own words for a moment, then continued. "Furthermore, I want you to arra

nge for me to be called in on a consulting basis."

Jensen blinked and bobbed his head nervously. "What? Elder, I can't."

Axen wasn't listening to him. "Mr. Panati will make it worth your while, and there are plenty of legitimate reasons. I've got more hands-on boptronics experience than anyone, and I helped build

the original Eden. I know everything about it, including plenty that you won't find on the blueprints."

Jensen's eyes narrowed. "I suppose, I could at least suggest the idea."

"Don't suggest, Jensen, push. Understand?"

Jensen nodded, and when it was apparent they were through with him, he straightened and tried to put on a business face. "It's been a pleasure, Elder." He stuck out his hand again.

Axen ignored it this time. "I'm not your friend, Jensen. I'm just somebody who is paying you for certain considerations. Remember that."

Jensen quickly pulled back his hand, slid past Panati, and departed without another word.

Axen and Panati watched him go, waiting until he'd been out of sight for several minutes.

"So," said Panati, breaking the silence, "what did you see during the disaster that told you this radiation thing was a cover story?"

Axen turned and started walking away. "I heard a woman warning me of the danger with her last breath." He kept walking. "I watched her melt into goo right in front of my eyes."

d # # #

Axen stood in the darkened control room of the Command Center studying the images flickering on the large screen. The link to the observer satellite had only recently been restored, and until now, acces

s to it had been so restricted that Axen hadn't seen a single image.

Now his dealings with Jensen had finally borne fruit. It didn't bother Axen that the call had come at such a late hour. He knew he was a controversial figure, and they undoubtedly wanted to keep the meeting as low-profile as possible. He noticed that m

ost of the CC staff had been sent on break, though Panati's friend, Della, was working on the far side of the room. Even she was careful to keep her eyes to herself, spending her slack m

oments looking out the large window behind her console rather than at them.

Jensen was there, along with a few other scientists that Axen knew only in passing. He noticed that Jensen was standing at the far side of the group and was being careful not to make eye contact.

He's not a good liar, something I should have taken into consideration before I started this. Still, he seems to have pulled it off.

On the screen, the same one-minute satellite video clip played for the fifth time since he'd arrived.

Dr. Kolo addressed Axen. He was the senior of the scientists, a hard-featured man with a shaved head and bushy black eyebrows. "Elder, we're consulting you on this matter because of your broad knowledge of boptronic systems. This video was taken from t

he observer satellite about a week ago. What do you make of it?"

As he watched, a vehicle, one of Nguyen's combat units, lurched its way across the screen, pausing to fire at a Cargo Truck that had similarly lurched within range. Parts of the truck's cab

glowed and melted under the combat unit's energy weapon, and the truck lurched once more before halting, perhaps forever.

He looked Kolo squarely in the eye. "I'd be glad to help if I can, but to be of much use, I need background. Tell me what really happened during the disaster."

The scientists looked uncomfortably at one another, then Kolo sighed and turned back to Axen. "The situation was a great deal more serious than we've led people to believe, and the danger is not past. Not even our continued migrations have ended it.

"We know that some kind of biological agent was released in the lab explosion, possibly was even the cause of it. That agent continues to spread deep underground in a manner we don't fully understand, seeping to the surface through vents where it can inf

ect buildings and vehicles.

"That agent was an accelerated microbe, engineered from bac

teria that had lived in hot springs, deep ocean vents, and buried in the crust of old Earth, microbes that already could survive the most hostile conditions imaginable. That microbe was designed to live in New Terra's crust, breaking oxygen bonds, releas

ing water and gases to form a new atmosphere.

"But," interjected Axen, "the human body is full of oxygen bonds. So are many of our plastics and synthetic materials. So are biological elements of our boptronic systems."

The scientists again looked uncomfortable. He was telling them things they hadn't intended for him to know. Just as well, as his conclusions depended on that knowledge.

Axen continued, "What we're looking at here isn't a boptronic system anymore, it's an optronic system. From what you've told me, the protein computer cores of these units are gone. It's as though they've been given a skillful and selective lobotomy."

Jensen looked at him. "You think they're dangerous?"

"Based on what you've shown me, I know they are. The combat units

are, anyway, and there may not be anything else left operational by the time you get there. These units have lost their higher command functions, their ability to understand and respond to commands, and to recognize other units, friend or foe. What's l

eft are hard-wired autonomous functions

move, steer, avoid, attack, patrol. My guess is they'll fire at anything that moves within range of them. Or, if erosion of plastic insulation is also causing shorts in their electronics, they could be totally un

predictable, capable of doing almost anything."

The scientists looked at one another, and moved off to a corner of the room and began talking in whispers. Axen hoped that they didn't give up on the idea of an expedition to the disaster site. It might well be their only hope of learning enough to cont

rol the Blight.

Axen watched the attack on the truck again, and wished that there were a way to connect Kraft to the satellite station so he could attempt to contact Emma. Plymouth had to be warned if they

had any hope of survival. They were in at least as much danger as Eden, perhaps more so, given their own ignorance. If they even still exist , he thought.

He longed to redirect the satellite to check on Plymouth, but that was impossible, and if the scientists had already done so the images were, like so much other information these days, being suppressed.

The scientists moved back over. "Elder," said Dr. Kolo, "we'd be grateful if you took the video and materials we've provided you and prepare a report

for us, outlining the danger you think those rogue vehicles represent, and suggesting countermeasures. We'd also like your input on procedures to transfer data from the non-biological memory systems of the Eden lab computers."

Kolo hesitated before continuing, long enough that Axen almost thought the conversation was over. "There is another, related, matter. The digital backups of our Gene Banks were in that lab. It

s possible that they've survived, and if so, we'd like to find a way to salvage the data."

Axen's eyes widened. "You don't mean that we've lost our Gene Banks?"

Kolo smiled slightly. "No, no, nothing like that. The three biological Gene Banks human, agricultural, and biosphere

are still intact. In fact, they're stored here in the CC for safekeeping. But the digital encoding of those genes has always been o

ur backup in case something ever happened to them, and we don't have either the time, the resources, or possibly even the technology, to reproduce the data."

Axen was aware of a spot to apply leverage. "Plymouth has their own copy of the biological Gene Banks."

Kolo's face was blank as he responded. "I don't see how that could do us any good. We aren't even sure if Plymouth still exists."

Oh, he's sure. He knows something about Plymouth that he isn't telling. If only I could get to that satellite downlink. But even the fact of Plymouth's survival or destruction was still denied him.

d # # #

As he left the CC he passed the parked scooters without stopping, deciding that the walk would help him think. He'd gone only a hundred meters or so when he ran into Panati. "What are you doing here?"

Panati frowned and looked away. "Della's getting off shift and she and I were going to play a game of billiards."

Axen just shook his head.

"It's none of your fragging business, Axen."

Just then they heard a muffled boom, the air around them seemed to surge toward the CC, and Axen felt his ears pop. He was already looking for the nearest emergency suit locker when it was apparent that the pressure drop was small, and only momentary.

Their fight forgotten, Panati stared at Axen. "What the frag was that?"

"Pressure blow-out," said Axen. He pointed toward the CC. "That way."

Panati ran up the tunnel, and Axen trotted after him, following him up the ramp toward the CC. As they watched

, a cart with a volunteer emergency crew rolled past, stopping just in front of the sealed emergency lock. One member of the Disaster Instant Response Team checked the indicators on the lock, then flipped the override switch that opened both doors. Peop

le began to stagger out, including Jensen and the scientists Axen had just been talking to, stunned looks on their faces.

Panati grabbed Jensen and pulled him aside. "What happened up there?"

He stared blankly at Panati for a moment. "Brook. Sorry. I just made it out in time. My ears are ringing. I don't feel so good."

Panati put his hands on the man's shoulders. "We'll find you a medic. What happened?"

Jensen put his hand over his mouth, fingers pointing upwards, touching his upper lip. "Is my nose bleeding?"

Panati gave him a little shake. "What happened?"

"View port blew out. Meteor hit I'd guess. There was a crack and then it just blew. A woman was standing right next to it and was sucked out."

"Woman? What woman?"

Axen's jaw clinched. He already knew the answer.

Jensen stared off into space blankly. "Della. Her name was Della.""###,
    },
    NovellaChapter {
        id: "s_e04.rtf",
        title: r###"Chapter 4: Autopsy"###,
        is_eden: true,
        chapter_num: 4,
        text: r###"Brook squirmed uncomfortably in the chair. The walls of the room were pastel pink, planter boxes were under all the windows and freestanding pots were jammed with growing plants. EnterCom screens on all the walls displayed decorative images of babies and

children at play.

Like most locations in the still-crowded colony, the room did double duty, as witnessed by the folded med-station in the corner. But the knowledge that on another day it might serve as a delivery room or emergency surgery did nothing to relieve the relen

tless cheerfulness of the place. The Nursery was a fragging strange place to come to talk about a dead person.

The small, blonde woman, Dr. Van Dozier by the name on the door, who studied him from behind her neat desk, seemed to sense his uneasiness. "I

know this is a little strange, Mr. Panati; autopsies aren't our usual business here, but the Medical Center is still coming up to speed and some nonessential tasks are routed to me. You'd made an inquiry about Della Ricca's accident, and given that she h

as no surviving family, I thought it would be appropriate to talk with you."

"Autopsy? She died months ago."

She nodded, and her green eyes seemed to look into him so deeply that it made him uncomfortable. "Her body was put into stasis until we had enou

gh slack time to examine it." She waved her hand to indicate the room. "Our primary business here is life, not death."

She saw him flinch. "I don't mean to be insensitive, Mr. Panati. In fact, the delay is the primary reason why I asked you here personally rather than simply sending you a message. If you don't mind my asking, was Ms. Ricca a companion, a lover?"

He smiled slightly and shook his head. "We were friends, classmates in school. We'd been spending a lot of time together lately, and I guess I wondered if it might turn into more, but

well, we'll never know about that I guess. You learned something about what killed her?"

Dr. Van Dozier looked down at her desk, her mouth a tight line. "Nothing you couldn't have guessed. Massive trauma resulting from explosive decompression and the fall from the window. Oxygen deprivation only quickened things I'm afraid. On the other h

and, if it's any comfort, she didn't suffer. The loss of consciousness would have been almost immediate."

None of which came as any surprise to Brook, which made his presence here all the more puzzling. "That's all?"

She chewed her lower lip. "The only solid thing, yes. But now that I've had the chance to study the accident report, I'm left with questions, probably the same questions you had when you made your inquiries. Only a small meteor impact would have left n

o evidence, and yet a small impact shouldn't have shattered the window that way. At most, there should have been a small crack that the self-repair gel could have sealed."

He leaned back in his chair, surprised. "That's exactly right. I have my resources, but the Disaster Instant Response Team that handled the investigation has been very quiet about the whole business. I smell cover-up."

"Which I'm not a party to, I assure you. We coordinate with the DIRT agency, and I have a number of contacts there. It's just possible I can find out more for you, if you want."

"Want? I'd be very grateful. Please."

She let out a little sigh and broke eye contact again. "Which brings me to another matter. Rumor has it that you're a man who trades in favors."

His delight went out like a match in vacuum. "You mean, you want to exchange a favor for information on my friend's death?"

She held up a hand to calm him. "No, no. I'll find out what I can in any case. But there is a small bureaucratic matter that you might be able to influence, and if you could, well, I would be in your debt."

Brook untensed. He realized that he didn't want to be angry at this woman. "I can try. What do you need?"

"The colony's human Gene Banks have been located in the Command Center since the colony relocated. Traditionally these have been kept in the Nursery and I assumed they'd be moved here as soon as we were fully operational. Somehow, though, the transfer s

eems to be hung up by red tape.

"Rebuilding the colony population is important work, and right now I or one of my staff has to go to the CC every time we need a new genotype. It's a huge bother and a waste of our time, and you can see how backed up we are."

Brook thought about it. His remaining contacts in the CC were limited, but Della had had friends there, and he knew they'd want to aid the investigation if possible. "I think I can do something about that." He stood and held out his hand to her. She t

ook it, her fingers soft and surprisingly strong.

Her smile was back. "We should meet again to compare notes, but if there really is something suspicious going on, perhaps we shouldn't meet here." She seemed to con

sider the idea for a moment. "I just got assigned my new quarters, and the woman I share with works evening shifts. Perhaps we could have dinner there."

He sighed. Three times already he'd used his own room assignments as trading fodder, and he was still sleeping in a corridor. "That would be great, Dr. Van Dozier."

"Please, call me Echo, if I can call you Brook." She watched his face for a reaction, as though she'd been expecting one. "It's a funny name, but I like it." He wasn't sure if she was talking about her name, or his. "I'll message you when I have somet

hing to share."

He smiled back at her as he headed out the door. "You do that."

d # # #

He'd arranged to meet Axen at a tunnel junction near the south edge of the colony. Most of the buildings were still being completed, and thus traffic was light this l

ate in the day. He found Axen, waiting, impatiently as usual, even though he was only a few minutes late. As he walked up, Axen was staring at a symbol spray-penned on the wall, a red circle divided down the middle by a vertical line.

Axen looked over his shoulder and glared at Brook. "Where have you been?"

Like it was any of his business. The "old man" was getting his fingers far too deep into Brook's affairs, and Brook didn't like it. Thus he took a certain perverse pleasure in telling a half-truth

. "I was doing some trading. I made a new contact in the Nursery and I'm greasing the wheels for the day we need something from them."

This seemed to placate Axen. "Well, then. It's just that you've been so distracted since that Della girl died. I know it upset you, but I have to be able to depend on you. I warned you about..."

Brook scowled at him. "You know, you can be a cold son of a 'bot sometimes, Axen. She was just a friend, though that's plenty reason enough for me to be upset, but my personal life is my

personal life, and I expect you to keep out of it."

Axen frowned, but said nothing.

"And you can depend on me. Kraft was transferred to the new Robot Command Center this morning, installed in the rack that you tweaked. I have a ConVec set to go at the Structure Factory. You want to go for a ride?"

Axen actually smiled. Brook wondered if his face was going to crack. As they made their way to the Structure Factory, Axen filled him in on his latest meeting with the scientist Jensen. "Se

ems they have a vehicle design that is able to handle the microbe. They've had one all along basically. Nguyen's prototype combat vehicle was hardened against biochemical attacks. The special inert seals have been adapted to other vehicles as well."

"Unmanned, I assume."

Axen nodded. "Whatever goes in can't come out because of the contamination. The idea is to find out as much as they can about the Blight and the accident at the lab. The biological components of the computer systems have been destroye

d, but some of the electronic and optical memories may still be recoverable. They haven't told me when the mission is scheduled to start. It may have already happened for all I know."

They slipped quietly through the Structure Factory, suited up, and made their way out through the service lock adjacent to the loading dock. The big Construction Vehicle was already loaded with the huge slab that was a structure kit.

They adjusted their suit radios to scramble for privacy and climbed into the cab. Brook

explained. "This is the Residence I pushed up the schedule for a few weeks ago. It has to go all the way around the colony. We should have an hour or more in the vehicle without interruption."

Brook finally had managed to get Axen his computer back, after a fashion. During construction, he'd arranged for Axen to get access to a computer rack in the Robot Command Center. Since the building hadn't then been operational, he could work without su

spicion. Only after the tampering had been done had he arranged for the Elder's Savant to be transferred to the new building and that particular rack.

The RCC was, like the CC, a highly secure area. Axen wouldn't be able to get back in without close supervision, but they'd opened a secure line that was unlikely to be detected. Axen had piggybacked a channel on the guidance signal being routed from the

RCC to the vehicles, and he could tap it through any vehicle's computer. And Brook certainly had access to vehicles.

Axen was tinkering with his radio again. "That symbol on the tunnel wall, have you seen it before?"

"No, should I?"

"I've seen three of them in the last week. The maintenance crews clean them up as fast as they appear, but now that I know what to look for, I've seen signs of a dozen more that have been painted over."

"Kids?"

"Possibly -- most likely, but I'd just as soon know what it means. It looks a bit like the Plymouth seal. Or possibly it represents a divided world, or a world reunited."

Brook thought of his visit to the Nursery earlier. "Or a dividing egg. It could be kids, or a political faction, or Plymouth sympathizers, or a band of rogue artists brightening up the colony. I couldn't care less."

Axen finished his tinkering. "I've routed the channel through another scramble to my suit radio."

Brook grimaced. "Again, you don't trust me."

"I tell you what you need to know." Then he touched a suit control pad and the radio went silent. Brook could see his lips moving as Axen talked with the computer, but could hear nothing.

Brook leaned back in his seat, put his feet up, crossed his arms over his chest, and sulked. He watched the panorama of the colony skyline as it rolled past the windows. Ten minutes later, Axen tuned his radio back to their common channel.

Brook studied the Elder's face. His features looked drawn and tense. "Well?"

"It is essential that we get Kraft tied back into the satellite uplink. Drop everything else and get on it."

"Why?"

"I'll tell you when you

need to know," Axen snapped. "Just get me the fragging link." He seemed to think about it for awhile. "I wouldn't make any long-term plans for this place if I were you. I wouldn't make any long-term plans at all." Then he switched his radio back to t

he private channel and ignored Brook.

Brook stared at him silently. What in blazes did he mean by that?

d # # #

She'd insisted that they eat before she told him what she'd found. At first he'd resented the delay, but as he sampled the chow mein she'd pre

pared, probably only the second or third home-cooked meal he'd had in his life, and they shared the beer he'd brought, things seemed less urgent.

He genuinely enjoyed Echo's company, though he felt a little guilty, as though he were somehow taking advantage of Della's death. Also, in the back of his mind, was always Axen's voice, warning him of the perils of relationships. Brook wasn't sure if he

wanted to embrace that advice, or defy it, though as the evening went on he was leaning toward the latter.

S

he told him about her childhood at an isolated research station away from the main Eden colony, which is why he hadn't known her, even though they were the same age. She told him how she'd rarely had any playmates and how, when she'd learned what her nam

e meant, she'd made up a pretend playmate, an identical twin.

"What did you call her?"

She giggled. " 'Echo,' of course."

Of course.

She explained how this led to her interest in genetics and reproductive medicine. "I was a lonely kid. I have a thing about making sure there are no more lonely kids out there."

"Speaking of which, I have a lead on getting the Gene Bank transferred to the Nursery. I've talked to friends with Council influence. Seems some of the Council think it's safer in the CC. The paranoia level is getting pretty extreme around here, but th

ey're in the minority and don't want to make a public stink. I wouldn't be surprised to see the paperwork go through in the next week."

She smiled warmly. "Thanks, Brook, that's great news. Keep me informed of the details, okay?" The smile faded and she licked her lips nervously.

"I have some news for you too, Brook, and I have been putting it off. I just wanted a chance for you to get comfortable with me first, to develop some trust. This is the kind of news that, coming from a perfect stranger, you probably would be reluctant

to believe."

Brook put down his beer and leaned closer. "Okay."

"I talked to my contacts in the DIRT. Sure enough, there is a cover-up. They aren't protecting anyone. Quite the contrary, they're trying to avoid a panic.

"They found evidence of tampering on the window. Those things are a sandwich, two panes of armor-glass with a layer of sealant gel in the middle. Somebody tampered with the gel. It could have been done in any number of ways, from inside or outside the

window.

"Suffice it to say that somehow they injected a chemical that over time converted the ge

l into a highly unstable, if not very powerful, explosive. A sharp tap is all it took to set it off, and it blew the window to shreds, sucking out whoever was standing in front of it. Any kind of projectile weapon would have produced a sufficient impact

, even a slingshot, maybe even just a well-thrown rock."

Brook blinked. "You're saying she was murdered."

"I think someone waited outside until they saw her through the window, then set off the explosion."

"Why would anyone do that to Della?"

She avert

ed her eyes. "You knew her better than I. Since we lack a formal police force, the DIRT is handling the investigation. They're blowing it. They're investigating her friends and co-workers, though they're not being very thorough about it. For instance

, near as I can find out, they completely missed you."

"We had only gotten back together in the few weeks before she wait a minute! Are you suggesting you think I killed her?"

Echo laughed. "No, of course not. But it means they've missed a number of

connections to her that wouldn't otherwise be apparent." She paused again, searching for words. "I've stumbled on other evidence that the DIRT seems to have missed. Putting that together with what I know about you leads me to one possible conclusion th

at they'd never consider."

"Which is?"

She sighed. "Have you ever considered the possibility that your friend Axen Moon is a murderer?""###,
    },
    NovellaChapter {
        id: "s_e05.rtf",
        title: r###"Chapter 5: Repercussions"###,
        is_eden: true,
        chapter_num: 5,
        text: r###"Axen crouched down behind a control console and snapped an access panel back in place. He still made sure his duties brought him to the Command Center frequently, even though Kraft was now attached to the RCC. It was a good place to eavesdrop and observ

e, where an occasional nugget of information could still be found.

Moreover, his political stock seemed to be on the rise, the Council less interested in his movements, less concerned about his resources. More likely, however, they were just distracted, or simply too busy to bother with him.

Axen was distracted too. New Terra was doomed, or so the scientists believed. They hadn't told him this, of course. They'd thanked him for his help, smiled their false smiles, and covered up everything, even Jensen. Especially Jensen.

But Kraft knew everything they knew, and more. Being separated from Axen seemed only to have made the amazing computer more resourceful, and it

s time in the CC had given it unprecedented access to information. Its position in the RCC had allowed Kraft a front-row seat for the old-Eden expedition. Now, everything Kraft knew, Axen knew as well.

The Blight was spreading too rapidly through the planet's crust, taking water out of the rocks, lubricating long-locked faults, creating geological instability, surging forward on the wave of hot ground-water that it had created, repeating the cycle. Not

hing short of nuking the entire planet down to the m

olten mantle would kill it all, and nothing less than killing it all would be effective. There was still token research being done toward creating a self-replicating counter-agent, but nobody gave it any serious chance of success.

The only hope now was to stay one step ahead of the Blight, gathering resources as they went, to reestablish orbital capabilities, to build another starship, and to find another planet to settle. He slumped back against the console and shook his head sad

ly. We've come full-circle. Who knew it would lead to this?

Certainly not he. Certainly not his fellow Elders when they decided that the colony's best chance for survival was to split in two. Now, instead of splitting things apart, all he wanted to do was hold things together, to put back together what they had

broken asunder.

But things only showed more signs of flying apart. He'd seen more graffiti this morning; he saw it almost ever day. And he'd heard a name whispered. Masters. What did it mean?

Then there was Panat

i. The young man was cold and distant and Axen couldn't figure out why. He avoided Axen, seemed careful never to be alone with him. Panati was still responsive when Axen asked for access to a vehicle so he could communicate with Kraft, but Axen didn't

know how much he could be depended on, or trusted.

As for the why of this sudden change in behavior, Axen couldn't understand it. There was another woman, of course, a doctor from the Nursery, but that didn't explain anything.

He heard voices, and the so

und of men grunting under a heavy weight. He peered over the top of the console and was surprised to see Panati across the room. He and a pair of workmen were moving a cumbersome and heavy metal cylinder from which wisps of frosty gas emerged.

Axen recognized it as one of the Gene Banks, the frozen repositories of Earth's genetic heritage. Along with the workmen, a man and a woman carrying side-arms and wearing the navy blue arm-band of the Volunteer Guard watched over the transfer. Axen wond

ered if this had something to do with the doctor from the Nursery. If so, it seemed a curious kind of favor to be granting. Where were they going with the Gene Bank?

Axen thought about simply asking, but he didn't think he'd been seen, and some instinct told him to duck back down and keep out of sight. Whatever Panati was doing, he was doing it in plain sight of everyone in the CC, so he obviously had authorization.

Still, it didn't smell right. Their efforts to recover the digital backups of the Gene Banks fro

m the original Eden had come, as Axen had predicted, to nothing. Two of the big quantum crystal memories had been located, but they couldn't be brought out without contamination, and the information stored within was too vast to transmit from the contami

nated area. Axen couldn't imagine why anyone would mean the Gene Banks harm they were irreplaceable.

He considered just staying put and minding his own business, but it wasn't his nature, and despite himself, he was concerned about Panati. Frag him, what's gotten into the boy?

He couldn't hear the workers anymore, but just in case, he stayed put for awhile before emerging from his hiding place. He glanced at the door. Panati and the Gene Bank were long gone, but he might still be able to catch up with them, perhaps learn what

Panati was up to. A pale-skinned technician looked up from her work and glanced at Axen curiously for a moment. "Was that a Gene Bank I saw them moving?"

She shrugged. "Looked like it. Not my department."

"Do you know where they were moving it?"

"Not my department."

Axen grunted and walked out the door. It wasn't that he thought Panati was up to anything, it was just that the graffiti he kept spotting had raised his paranoia level. The last time he'd seen anything like it was during the political schism that had cr

eated Plymouth, and during that revolution, it had been he and his fellow Elders doing the painting.

He made his way out of the CC and picked up a scooter at the tran-station. Doubtless they were moving the Gene Bank in a cargo cart and he should be able to catch up with them, if he was correct in assuming they were headed for the Nursery.

He gunned the scooter, making the wheels squeal on deck-metal and roared off after them. They had a strong head-start, but the tunnel traffic was light this late in the evening and he made good time. He arrived at the Nursery only to find it locked up f

or the night. Could he have been wrong about their destination? He didn't think so.

He'd followed Panati's normal route from the CC to the Nursery, but a new area of construction had created a possible shortcut, one that Panati would be very familiar with, given his work.

He spun the scooter around in a tight circle and backtracked. It was immediately apparent when he entered the new tunnels. Traffic was nonexistent. A thin layer of reddish powder dusted the walls and other exposed surfaces where traffic had not brushed

it aside. There was also the characteristic chemical "new building smell." It wasn't unpleasant, but it was a minor health hazard. Years of research had failed to eliminate outgassing from new synthetic materials.

He rounded a corner and almost ran head-on into the cart. He spun the scooter and laid it down on its side, sliding to a stop just short of the wreck.

The cart was sitting at an angle, its nose crunched against a tunnel support rib. The driver, one of the workmen, was slumped over his controls. Panati was face down on the deck near the empty passenger se

at, from which he'd probably fallen. The others were sprawled around the vehicle, motionless. Axen's hip twinged as he stood, but he ignored the pain and made his way to the male guard, the nearest of the five.

He knelt next to the man and touched his neck. There was a pulse, and the man was breathing. While the others didn't appear to be injured, the guard was face down in a small puddle of blood. Axen rolled him over and saw the side-arm still clutched in h

is hand. Axen took the gun and pulled back the collar of his shirt and examined the wound. A soft-slug had pierced the right shoulder, doing considerable damage, but it didn't seem immediately life-threatening.

Then he spotted a small fire extinguisher near one tunnel wall, and noticed for the first time that there was something odd in the air beyond the new building smell, a pungent and sickeningly sweet odor. He looked again at the extinguisher's tank. They'd

been gassed.

He could reconstruct the scene in his mind. Someone, possibly in a pres

sure suit or breathing mask, had hidden along the tunnel in wait. As the cart approached, they'd flooded the tunnel with gas. The driver had been affected first, and the cart had crashed, but one of the guards had fought off the gas long enough to stagg

er out of the cart and pull his weapon. The guard had been shot for his trouble. Axen noted that the gun in the man's hand hadn't even been fired.

He looked at the cart again. The cargo bed was empty. They'd stolen the Gene Bank.

Now he knew he was ou

tnumbered. It would have taken at least two people to lug the gene bank out of the truck, and it would be logical to have at least one more as a lookout. He lifted the scooter, fired it up, and made his best guess at the direction they might have headed

. His guess was deeper into the new construction zone. There would be few people to see them, and assuming they were wearing pressure suits, there were dozens of emergency and service airlocks through which they could reach the surface. For that matter

, every incomplete building also offered easy access to outside.

He paused long enough to activate the nearest disaster alarm box, then rushed headlong into the dusty tunnels. He didn't expect to find the hijackers, given their head start. His only hope was that the cumbersome Gene Bank would slow them down.

As he approached a tunnel junction, he spotted a silvery object lying on the deck. He stopped the scooter short of the junction and leaned it against the wall. He cautiously walked over and picked up

the object, retreating quickly into the cross-tunnel so as not to be spotted. He rolled the brushed aluminum cylinder over in his fingers. A paint-pen. Red.

The tunnel to his right dead-ended a few hundred yards on, and the normal overhead lights hadn't been installed yet. Only a few scattered work lights set up amid the scaffolding pierced the gloom. He pulled the gun out of his pocket and, keeping his bac

k close to the wall, inched forward. He heard voices, and an intermittent hissing sound. A paint-pen.

He spotted the pressure-suited figure at the end of the corridor, hanging off a scaffold, putting the finishing touches on an enormous version of the divided circle he'd seen before. The circle covered most of a large set of pressure doors leading into t

he uncompleted factory module.

He crept closer. The suit's visor was closed now, but he'd heard voices, so there were others around, possibly carrying the Gene Bank. Could they already be on the other side of those doors, or were they simply invisible in the shadows?

He stepped out of the shadows and pointed his gun at the suited figure. "Don't move! I've got a gun."

The figure let go of the scaffolding and dropped into the shadows. Axen let off a wild shot but knew he hadn't hit anything. He hadn't used a gun in years, and he'd have to be much closer to have a prayer of hitting anyone. Unfortunately, he couldn't c

ount on his adversaries being as unskilled.

Ducking low, he moved closer, using scaffolding and piles of construction material as cover. He popped his head up, and someone took a shot at him, but he had time to make an important discovery.

The Gene Bank was sitting on the deck right in front of the spray-painted symbol.

That was good news and bad. It meant he might yet have a chance to recover it, but it also meant that whoever had been carrying it was now no longer so encumbered. He moved closer still, and popped up long enough to let off another round, just to keep t

heir heads down.

He was back under cover by the time they

responded with return fire. He popped again, and spotted a pressure suit helmet only a four or five meters away. He squeezed off a shot and the soft-slug pancaked against the plating with a metallic splat just to the right of the figure.

Three more shots in his direction, all wild. I might have a chance,

he thought. Then he heard a clank, and the sound of heavy machinery. A sound like a tornado nearly deafened him, and a blast of wind pinned him in his hiding place and blinded him with clouds of dust. They've opened the pressure doors.

He clinched his eyes shut and tried to recall the appearance of the doors where the control panel would be located. He'd only have one chance at this. He pulled himself around his barricade and let the wind carry him toward the door, trying to veer hims

elf enough to the side to hit the controls.

A slab of insulation smacked him painfully behind his right ear, and he felt hot blood running across his neck. He slammed against the right door, the control panel just ou

t of reach. Through the open doorway he'd seen a flash of the building's dark skeleton silhouetted against a starlit sky. He felt the wind tearing at him, and clawed his way toward the panel.

Hot needles stabbed in his ears as the pressure dropped. He pushed himself forward the last inch and slapped the emergency close icon. He didn't hear the doors closing, and was only aware when the roar of wind stopped.

He slumped to the deck and tried, with limited success, to catch his breath. He looked around. They were gone, out through the doors while he'd been struggling to survive. Then he spotted the Gene Bank. It sat on its side near the door, half covered w

ith a sheet of light-paneling. He pulled himself to his feet and managed to tug the panel clear. What he saw crushed him.

He fell to his knees in front of the Gene Bank, which appeared to have been shot repeatedly at close range, close enough that even s

oft-slugs had ripped through its aluminum jacket. The last of its liquid helium coolant boiled away from a puddle on the deck as thousands of tiny glass vials spilled out through the holes and onto the floor, where they lay shattered and broken.

He looked up at symbol painted on the doors, and screamed in rage."###,
    },
    NovellaChapter {
        id: "s_e06.rtf",
        title: r###"Chapter 6: Alliances"###,
        is_eden: true,
        chapter_num: 6,
        text: r###"Emma's face looked shocked as it appeared on the heads-up display of his spacesuit helmet. He'd decided that for this first attempt at reestablishing contact, he'd risk a visual signal. "Maker, Axen, I was afraid you were dead."

The Cargo Truck hit a pothole so deep that Axen was tossed against his straps. The pain reminded him that he was still very much alive. "They took Kraft from me, Emma, and things have happened here. I don't know if you know this, but Eden is on the mov

e. I didn't know if you were alive either."

She flashed a little smile, and it sent a flutter through him. Had he missed it that much? "I can't say it hasn't been a near thing a couple of times, Axen. We've had our problems too, and I'm at least periph

erally aware of yours from satellite imagery. But though I've tried to contact you every chance I've gotten, there's never been an answer."

"I wish I'd had that kind of access, Emma. Even now I still have to talk to Kraft through backdoor channels. That's why I'm in a pressure suit and busting my spleen bouncing around in this garbage truck."

She smiled again. "It's good to hear from you, Axen."

"It's good to talk to you too, Emma, but my time is short and I don't know when I'll be able to talk with you again. I have to warn you about Nguyen's bug, the thing we call the Blight. I'm passing you a data file while we're talking."

"We know quite a bit about it already. Frost has been incredibly resourceful on the matter."

He took a deep breath before continuing. "Did Frost tell you the planet is doomed?"

Her face went dead serious. "You're sure?"

"There's maybe one chance in a thousand, but we have to assume that the only real hope for survival is to build another starship and escape the planet."

She nodded sadly. "We suspected as much and have been acting accordingly."

"It's going to be a close thing, a real balancing act of resources, research, and technology. We can't just recreate the original starship that brought us here, it was a brute-force project constructed with the resources of a fully developed planet behin

d it. We have to build smarter, faster, lighter, higher technology across the board."

He paused for another deep breath. This wasn't going to be easy. "Emma, I don't think there are enough resources for two starships, or two starship programs. Based on what Kraft has learned, Eden has a clear technological lead."

"What are you saying, Axen?"

"I want you to help hold back the Plymouth program in any way you can, and give Eden the clearest shot possible." He saw the anger in her face, though he couldn't predict how that would color her response.

"Is that all?"

"Plymouth has one thing that Eden needs for its program to be successful." Again he paused, just the thought of it

making his teeth grind. "Our human Gene Bank has been destroyed by terrorists. I want you help me to get the copy from Plymouth. With a minimum of bloodshed, of course."

"Oh, of course." She stared back at him. "Axen, were you always this oblivious to sarcasm? Frag if I'm going to sell out my own people. You may be ahead on the hardware, but without the human Gene Bank, how do you think any colony ship has a prayer of

success? And if you have terrorists running around who would do such a thing, how can you even imagine you should be trusted with the only other Gene Bank in existence?"

"The people in Eden are your people too, Emma. They're all our people, or have you forgotten that? I'm just trying to do what's best for the survival of the species."

She smirked. "Are you sure? Why don't you sabotage Eden's space program so Plymouth and its intact Gene Bank have a chance?" She waited for a moment, studying him. "I didn't think so."

This was getting nowhere. "Emma, there's an impact site about

fifty kilometers east of you where some of the wreckage from the original starship touched down. We have an expedition there now gathering the remains of key components so we can examine and reverse engineer them."

Her eyes widened. "Plymouth is sending out a convoy to do the same thing."

He groaned. "That's why I'm telling you, Emma. The Eden expedition is armed. Find a way to warn your people off."

"It's too late, Axen, they've already left." She licked her lips. "They're armed too. I guess this is about to heat up into a shooting war, no matter what either of us does."

He glanced out the window of the truck and saw the towering side of the Ore Smelter looming over him. "Emma, I have to go. Think about what I said."

"Think about what I said, Axen. May the best colony win. Pray that somebody does."

He shut down the connection just as the truck rolled onto the smelter's loading dock. He slumped back in the seat and gazed at the smelter's mottled wall as a thump indicated that the loading grapples had latched onto the trailer behind him.

That went well. And Emma didn't think he understood sarcasm.

d # # #

Brook dropped his ClipCom carelessly on a flat outcrop of rock and sat down beside it. He drew his knees up in front of him, leaned back against a larger boulder behind him, and settled back to watch the ConVec put the finishing touches on the new Consum

er Goods Factory. It was supposed to be a boost for colony morale, but it was going to make it that much more difficult to maintain his old wheeling and dealing, not that he'd had a lot of time or enthusiasm for it lately.

He was busier than ever these days, and that was good. The work kept him from thinking too much, dwelling on things he didn't understand, and possibly never would. How could Axen have killed Della? And did he?

On the face of it, the evidence was pretty damning. Axen had been in the CC just before the incident occurred,

and thus might have performed the sabotage. He objected to Brook's association with Della, and judging from her comments, she might have just made a point of coming between Axen and his beloved computer. He'd been surprised, and perhaps a little upset,

when he'd run into Brook outside the CC just before the explosion.

Of course, he'd been with Brook, and thus couldn't have been the one to set off the explosion if it happened as Echo had suggested, but there were other ways, and Axen might have had an accomplice.

On the other hand, though Axen had impressed Brook as many things, a cold-blooded casual killer wasn't one of them. But what did that mean? Would you know a killer if you met one? What does a killer look like? He didn't know.

But until he could find a way to resolve the matter one way or another, his close association with Axen was over. He still provided Axen with access to vehicles, and in turn, to Kraft, but he wasn't sure if that was out of doubt, courtesy, or fear. Perh

aps it was simply out of the lingering belief that Axen really was trying to work for the greater good of the colony, that for the moment, letting him continue to operate was less a crime than stopping him.

But there were doubts about that too. Though Brook had been unconscious at the time, some witnesses put Axen on the scene when the Gene Bank was hijacked and destroyed. Echo had even suggested that Axen might have masterminded the heist, though Brook co

uldn't imagine why. It was so hard to understand why anyone would have done it.

He was always aware of his responsibility in that event, and lived in dread of the ultimate consequences. There was talk of a raid on Plymouth to "liberate" their Gene Bank, and think as he might, the alternatives were few. How could the starship leave

without it, and the Plymouthers were unlikely to just hand it over.

He sighed. It was close enough to quitting time, and he wasn't going to get much else done in his present mood anyway. He slid down off the rock and picked up the clipb

oard. His dirt scooter was parked only a few meters away. He dropped his ClipCom in the small cargo box, straddled the seat, and whirred off in the direction of the Structure Factory, where he was supposed to meet Echo.

d # # #

Axen returned to his quarters and slumped wearily into the bunk. He looked around the room, sparse and undecorated. The place felt as empty as his soul. All this work, all this fighting, and what have I got to show for it?

He knew the answer even as he posed the question. His work was his life, for better or worse. He had to see this thing through, and then... what?

Never mind that. He couldn't see the end of the current situation, though there had to be one, in death or in triumph. It was just that right now, he felt very, very tired.

He thought about zoning out watching the EnterCom, but decided to check his mail first. The first item caught his immediate attention. It was from the Council, a quarters reassignment. He read the text carefully, then read it again.

Then he stood up so quickly he nearly bashed his head on the cupboard over the bunk. "They can't do this!" All thought of rest was gone; he had work to do. They'd taken Kraft, kept him out of the loop, but nothing they'd done so far compared to this.

He was being exiled, and he wasn't going to stand still for it.

d # # #

Though both of them had their own quarters, the Structure Factory was still a favorite place to meet and talk. All factory storage bays were full in anticipation of the next colony rel

ocation, and it would be shut-down and quiet there, a nice trade-off for the loss of their usual mechanical floor-show.

He parked the scooter next to a service airlock, cycled through, stowed his spacesuit in a locker, and climbed the ladder into the tran-station just outside the factory.

As he approached the big utilitarian doors, he noticed the symbol painted there, the circle divided by a line. It was a small one, only about a meter across, but still it annoyed him. Who were these people?

The interi

or of the factory was dark and eerily quiet. His footsteps echoed off the metal walls as he climbed the long flights of stairs up to the observation balcony where they were to meet. When he reached the last landing, Echo was nowhere to be seen, but ther

e was another symbol painted on the front of a supply locker. How did they get in here?

"Brook?"

He turned to see Echo climbing another set of stairs on the far side of the balcony. He was surprised to see that she wasn't alone. She had two tall men and a stocky woman with her.

"Brook, these are some friends I've been wanting you to meet for a long time. I hope you don't mind that I've brought them along." She gestured toward the taller of the two men, thin of face and nose, with an equally thin mustache. "This is Jacque." S

he indicated the other man, slightly shorter and at least twice as wide, with a round face, ruddy skin, thick lips and a wide, flat nose. "This is Gi." And finally the woman. "This is Sharon." Echo smiled at him. "They're my

friends, and I hope they'll be your friends too."

Brook nodded without much enthusiasm. He'd been planning to spend some time alone with Echo, and wasn't in the mood for a party.

"Folks, this is Brook Panati. I hope he's our latest recruit."

Recruit? "Echo, what's this all about? I thought we had a date."

"You have a date, Brook, with destiny. This is an important day, the day you're invited to join the Masters."

"The what? " Actually, he'd heard the name in association with the graffiti, though he'd wondered if it was any more than wild talk.

Echo smiled patiently. "The Masters, Brook. We're a group that believes that the reins of power in Eden are in the wrong hands, and that we've been manipulated by the legacy of the Elders for too long. We want to take Eden back, Brook, to take our dest

iny back, to take our future back."

It all began to sink in. " You destroyed the Gene Bank, and you used me to get to it."

"It was an accident, Brook. We intended only to hold the Gene Bank and use it for a political exchange. We never intended for anyone to get hurt. That's why we used gas."

"That's why you carried guns," he responded quickly, expressing more anger than he'd intended.

"We're dealing with dangerous people, Brook, and we have to be prepared to return a deadly response if necessary, but we've never set out to kill anyone, even our enemies."

Gi pointed at the symbol on the locker. "We stand for strength, and unity of purpose. We wish to create a world of equals."

"Equals," she repeated. "We have high ideals, and work for a higher purpose. I don't blame you for being angry. We did use you to get at the Gene Bank; we did mislead you about the incident. Not 'we' I

did those things. But it was for a good cause, and we meant you no harm. It's just that you're a useful person, a resourceful person, and we wanted you on our side. We still do."

He frowned at her. "How can I join you, after what you've done?"

"The Gene

Bank was a mistake, Brook, but not a tragedy. We mean to use the starship to take our ideas to a new and better world, and we don't need the Gene Bank for that. We're clones, Brook, clones or the children of clones, though we don't think of ourselves t

hat way. We're hand-chosen, the very best of all the races on Earth. Why should we dilute that perfection with weaker bloodlines, mongrelize it with random cross-breeding, when we can simply clone ourselves and maintain the perfection for all time?"

She smiled and spread her hands. "Don't you see, Brook, we're the Masters, all of us, if we only have the will. And as for joining us, you already have in a way." Her smile turned coy. "The Masters have a leader, Brook. Me."

Brook just stood there, his mouth hanging open, not sure what to say. Great Maker, what have you gotten yourself into?"###,
    },
    NovellaChapter {
        id: "s_e07.rtf",
        title: r###"Chapter 7: Gulag"###,
        is_eden: true,
        chapter_num: 7,
        text: r###"Axen stood as the transport's airlock clanked against the mining outpost's Command Center. Motors whirred and pumps chugged as the locks sealed together and cycled. He took a final glance around the transport's empty cabin, the dozens of seats empty sav

e for him. One last bit of VIP treatment, he thought, my private taxi to hell.

The lights around the lock turned green, and the doors slid open with a hiss. He stepped through to find a broad-shouldered man with a handlebar mustache waiting for him. The man smiled and extended his hand. After a hesitation, Axen offered his own in

return. He wasn't feeling very friendly, but he might well need allies here, and he was cut off from all his familiar resources.

The man's grip was like iron, and he shook Axen's hand vigorously. "Greetings, Elder, welcome to the Gulag. I am Vox Borges, operations supervisor for this mining station. So, who did you offend to rate a transfer to this fine place?"

It was the sort of question Axen generally tried to avoid answering, so he said nothing.

Borges just laughed. "Come, we all have offended someone, Elder. There is no other reason to be here."

"It's a long list. A very long list."

Borges smiled broadly. "So it is, and myself, I had to offend only a single Council member to receive this 'promotion.' "

Behind Axen the lock doors cycled shut

and there was a clunk as the transport undocked. He noticed that Borges' attention was focused on the departing transport. "How are things back at mother Eden? I haven't seen it in a long time. Though I have accumulated much leave time, all the trans

ports returning to Eden are constantly full."

Axen's eyebrows raised. "That transport was empty. I was the only passenger."

"I'm sorry, my friend, but you are mistaken. I have seen the paperwork proving that it was full, no room for additional passenge

rs to Eden. Should there be any doubt, though you may not have seen him, there was a man in the cab with a side arm, ready to clarify the situation."

d # # #

Brook stared blankly at the young blonde woman who'd answered Axen's door. "Who are you?"

"I live here," answered the woman. "Who are you?"

"I'm looking for the Elder Axen Moon."

"I've never met him," she said, and started to close the door.

"Wait!" He half-threw himself into the door. "He lives here."

She put a hand on his chest and gently pushed him back. "Not anymore. I moved in a week ago. I don't know a thing about him. Now go away, before I have to call the Volunteer Guard."

He stepped back and composed himself. "Sorry. Sorry. Just a misunderstanding. He must have been moved. I'll take it up with the housing office."

But when he found a quiet corner and called the housing office, they didn't have a listing for Axen. The woman there was an unshakable bureaucrat, and couldn't be swayed. "He's an Elder, frag it. He can't have just disappeared."

She glared at him from the screen. "I've told you, I don't have that information. If he's an Elder, perhaps he simply doesn't wish to be bothered. Good day."

Brook stared at the suddenly blank screen. Could that really be it? Could Axen simply have pulled some strings to allow himself to adopt a lower profile?

Unlikely, and since Brook had arranged to put him back in touch with the Savant, he'd never gone more than a week before sending word that he needed access to a vehicle. Something was definitely wrong, and it couldn't have happened at a worse time.

He leaned back in his chair and watched the people lounging around the Residence's common area. None of them seemed to be watching him, but he couldn't be sure. He'd considered coming here at all a risky move, but these Masters probably knew more about

what had happened to Axen than he did. Certainly they didn't number him among their friends.

For the moment, Brook was playing along with them, providing them with information, trading small favors on their behalf. Often as not, they provided all the barter goods, simply using him as a front for the transaction. They seemed to think, rightly so

, that anything funneled through him was less likely to draw official attention.

What truly had him perplexed was Echo, sweet, open Echo, the leader of a secret resistance movement, thief, saboteur, and who-knew-what-else. How could it be?

She genuinely

seemed to believe in her future humanity, built on a few hundred "perfect" genetic genotypes, repeated over and over for all of history. He shuddered. This morning she'd proudly confided in him how she'd already created a clone of herself several years

before, using her position to switch her genetic material with a genotype from the Gene Bank selected by a young couple. Not only that, but she planned to do it again, soon.

Brook looked up as a woman walked past leading a small girl by the hand. She was the right age and hair color

but her eyes were brown. Brook dropped his head and sighed. He couldn't take much more of this. Echo was insane. That was the only explanation. He wondered if that insanity was genetic, and imagined it repeated endless

ly throughout history, like a hall of mirrors.

"Brook!"

He looked up to see Echo walking around a cluster of potted palms toward him, a big smile on her face. "I've been looking all over for you. You aren't avoiding me are you, naughty boy?"

She reached out her hand and he took it, feeling a little thrill at the contact, his mind and heart at war. For the thousandth time that day, he wondered if Axen was really right.

d # # #

Axen dropped his tool kit and lunged for a tunnel support column as the tran-station deck bucked under his feet. Scooters slid out of their racks and fell over like a row of dominos, and for a moment he was sure the hull would crack like an egg.

Borges held onto a window frame with one hand and grinned broadly at him.

Axen scowled at him as the shaking stopped. "Do you ever get used to this?"

Borges just laughed. He laughed a lot. It was really starting to annoy Axen. "In time, friend, you become thankful for each one that is just a tremor, not a major quake or volcanic eruption. Then there is the Blight, which is very close. At any momen

t one of our Robo-Moles could strike an infected flow, channel it straight to us, and the entire outpost could be overrun.

"This is a dangerous place, which is why people such as ourselves

are sent here. They say that if we mine our quota, we will be sent home. I don't think I believe that, but each day we live is a victory, and another chance that a change of power might bring us home.

What kind of change of power?

He wondered what Borges really wanted from him. He'd almost become Axen's shadow since his arrival, and Axen's new quarters were right next door to Borges'. Though Borges' title implied a great deal of responsibility, he never seemed to lack for time

to follow Axen around, working with him on the minor boptronic repairs and adjustments that had made up his current roster of assignments.

At first Axen had thought he was only bored, and hungry for fresh gossip about things in Eden, and so he'd gladly obliged him in hopes that he'd lose interest. He'd been especially interested in Axen's substantially edited reports about the Masters and t

heir activities.

Could Borges be one of them, or at least a sympathizer? Could that be the reason for his exile? Possibly, but certainly it was apparent that any number of offenses, real or imagined, could land a person here.

Axen picked up his tool kit and brushed himself off.

Borges frowned slightly and touched the communications unit in his right ear. "Have you been checked out in a spacesuit lately, my friend beyond our popular daily disaster drills, of course?"

"Yes, why?"

"The tremor seems to have caused a malfunction in Guard Post Three. Best we should check it out right away. Come." He opened the floor hatch to the side of the tran-station and began to climb down to the service lock.

Axen sighed. Even outside, there would be no losing his shadow.

d # # #

Brook sat in the cab of the Scout as it trundled around the colony on a fool's mission to the Garage. If he needed the time, he had the programming in place for it to travel all the way back to the Robot Command Center as well. If that didn't do it, wel

l, he suspected nothing would.

He'd watched dozens of times as Axen had activated the link to Kraft, even helped in its planning and installation. He thought he could initiate the link himself. Unfortunately, it didn't mean anything unless Kraft would talk to him.

He cleared his throat nervously and looked out the window at the Vehicle Factory that they were passing. What do you say to a computer?

Well, Axen made it seem natural enough, but Brook's direct interaction with the sophisticated, nearly human Savants was limited.

I know you can hear me." He paused a minute. Silence. "I know that normally you'd only talk to Axen Moon, but that's why I need to talk to you. Axen is missing." Again silence. "I'm sure you've noticed the unusual time lapse since your last communic

ation with him. I want you to help me find him."

Silence. "Kraft, I'm pretty sure Axen has mentioned me to you. I helped put you back in touch. It's only through me that you were able to communicate at all. Please talk to me."

Nothing. He sighed and scratched his chin thoughtfully. He felt like he was coaxing a shy child out of its hiding place. What would Axen do? Or, perhaps more important, what had Axen already done?

"Kraft, I know that Axen has had you working on the solutions to some very large, very complex problems concerning the Blight and the colony's survival. I'm sure you're still working on those problems at Axen's request, that it's an ongoing project. Isn

't that correct? Well, without Axen to continually provide you new informatio

n and to refine those problems, you have no hope of solving them. By talking to me you may be able to put yourself back in touch with Axen so you can solve those problems. Talking to me would be an extension of those orders."

The circuit was dead. Then there was a click, and an icon appeared on the display of his ClipCom, a moving assembly of gear wheels and cogs, like a translucent clockwork

Kraft's identity icon. "This is Savant Kraft. I will converse with you within the limits of my existing instructions. Where is Axen Moon?"

"I was hoping you might be able to tell me. He isn't in the colony, at least not officially."

"He is not in the main colony?"

What did it mean by that? "I don't think so. I haven't been able to locate him here."

"Then he may be at the mining outpost."

Brook blinked in surprise. "What mining outpost? I've never heard of such a thing."

"There do not appear to be official records, but I have deduced its existence through information I've gained access to recently. I ha

ve identified twelve structure kits and fifteen vehicles officially listed as recycled, yet this is not reflected in the output figures from the GORF. Conclusion: these items were not destroyed and are not being used in Eden. This suggests they are bein

g used elsewhere, and the mix of items involved suggests a mining installation."

Brook was amazed. It made sense, and yet, it would have had to have been done right under his nose. Only someone near the top levels of authority would have been able to pul

l it off. The audacity flabbergasted him. And if Axen had vanished to this nonexistent outpost, he probably wasn't the only one. How many of the people listed as missing during the last evacuation were actually political outcasts?

"Do you have any idea where this mining outpost is?"

"I have monitored weak vehicle control traffic at bearing 118 from the colony. I estimate the range to be one hundred and fifty kilometers. I do not believe these signals to be of Plymouth origin."

That was it then. He

knew where Axen was. Now all he had to do was figure out how to get him back and make it stick. "Kraft, it may be possible not only to get Axen back, but to put you two back together. I'll need your cooperation though."

"I will evaluate your requests. Please explain."

And he did.

d # # #

Axen shifted his weight on the service platform and looked at the status lights on the panel in front of him again. The Guard Post seemed to be working perfectly. The malfunction wasn't showing up on the local tell-a-tales, and therefore shouldn't have

registered at the CC either.

He looked down at Borges. "Are you sure this is the turret they told you to check? This thing looks fine."

"Perhaps there is a malfunction in the connections between the Guard Post and the CC."

"It's a radio link, just like the vehicle control signals. There's nothing in the middle to go wrong."

Borges shrugged. "Well, then, there isn't a problem. You should come down from there."

Axen muttered under his breath and closed his tool kit. He turned and backed down the ladder. "It is most curious," continued Borges. "Perhaps the problem is not where you thought it was."

Axen stepped off the ladder and turned to find himself looking down the barrel of a pistol. Borges was still smiling. "I am sorry, my friend, but there

are only a few ways to leave this Maker-forsaken place. I have been offered one, and you are about to discover quite another." He gestured at Axen's helmet. "If you would please loosen the fastenings on your helmet, we can end this rapidly and with mi

nimal fuss. I am told unconsciousness comes quite quickly."

He studied Borges, looking for some opportunity. The other man was standing in front of the powerful machinery that moved the turret. The door was on the far side of that mechanism. It seemed

hopeless. "No go, Borges. If you want me dead, you'll have to shoot me. I'm not going to make things easier for you."

He shrugged again. "So be it. There will be no questions asked about your death in any case, trust me on this." He raised the weapon to fire.

Suddenly there was a loud bang, and the Guard Post shook. At first Axen thought it was a tremor. Then the turret began to move. Behind Borges, huge gears and motors began to whir. The outpost was under attack!

Borges was distracted for just a moment.

Axen leapt, grabbing Borges' wrist with both hands and forcing the gun back over his head. The gun kicked, and the slug splatted against the ceiling. Borges put his other hand on Axen's faceplate and shoved him back.

Axen fell against the ladder. Borges again pointed the gun at him. Axen's hand groped for the discarded tool kit. He seized the handle and swung it as hard as he could. Borges threw up his hands to protect his head, and the case struck the gun edge on

, nearly tearing it from his hand.

Axen pushed off from the ladder and charged toward Borges like a football player. The man braced himself for the charge, but Axen veered, brushing past Borges, his real goal the door on the other side of the turret mechanism.

Borges was knocked backward into the moving gears. His helmet lodged between the teeth of two huge gears. They turned, then hesitated, then moved again with a crack and a jerk. Borges' helmet cracked like a walnut, and a cloud of gas spewed out. The t

urret moved the other way, freeing Borges, but it was too late. He grabbed weakly at his head, then fell to the floor and did not move.

Axen started to relax, when a force like a huge club knocked him into the near wall. He couldn't tell if the flashes he saw were real, or if he was only seeing spots.

Another explosion shook the Guard Post, only this time not as close. There was a rending screech of metal and the whole turret mechanism slumped away from him. The door was blocked.

He shook his head to clear it, then crawled forward on hands and knees. There was light; there had to be a way out.

He scanned the room. A hole gaped in the wall next to the platform he'd been standing on only minutes before. Weakly he grabbed the sides of the ladder and climbed it one painful step at a time.

He reached the platform, then used the railings to pull himself to his feet. The railing in front of the door was bent and twisted, and it was easy enough to squeeze past and through the hole.

He rolled over the lip of the opening and out onto the platform that surrounded the Guard Post. A row of Plymouth combat vehicles was rolling toward him. One of them he could see dead nose-on, and it seemed to be approaching quickly.

He scrambled to his feet and crabbed along the side of the defense turret and around the corner behind it. The light combat unit rolled past no more than a yard away.

Another explosive near-miss sent him scurrying away from the turret. Even damaged as it was, it was a prime target. He staggered across open ground, trying to keep out of the way of the circling combat units, but they seemed to be everywhere.

A missile flew over his head and exploded into the Rare Ore Smelter, then another, and another. The Plymouth units were moving deeper into the outpost, so the safest place for him seemed to be away from it.

He glanced at his suit status indicator to check his oxygen supply, but the indicator panel was fractured and dark. Frag. Well, he'd just have to take his chances. There was a clust

er of boulders two hundred meters from the edge of the colony. It would offer cover until the battle was over.

Halfway there, he started getting lightheaded, and dark spots crept into his vision. Something more than the suit's status panel must be damaged. It was getting difficult to walk, and his concentration was fading. He just wanted to lie down and sleep.

He let his eyes close for a moment, and when he opened them, a vehicle had rolled up a few yards in front of him and pulled to a stop. Puzzled

, he looked up at the Scout. For a moment, he thought it must be a Plymouth unit, but the markings were clearly Eden. The hatch opened and a spacesuited figure waved him inside.

He grabbed the ladder, took a step up, then lost his balance and fell back. The figure reached down and grabbed the hanging straps on the shoulder of his suit, and managed to drag him up on to the vehicle's fender, then through the hatch. He helped as m

uch as he could, but that wasn't much.

The hatch slammed shut and the vehicle made a fast turn and started bouncing across the landscape full tilt. He lay on his back, gasping for breath. Behind him, some lights turned green, and he tried to remember what that meant.

The other figure fumbled with his visor and managed to get it open. The air that rushed in was cold and slightly stale, but he didn't care. He sucked a deep breath in and coughed.

He rolled over, and found his face only centimeters from the flat side of a Savant computer housing. He glanced up at the figure, which was just now removing its helmet.

Panati grinned weakly at him. "Some welcome party you throw, Axen. Aren't you even going to say hello to Kraft and me?""###,
    },
    NovellaChapter {
        id: "s_e08.rtf",
        title: r###"Chapter 8: Siege: Resurrection"###,
        is_eden: true,
        chapter_num: 8,
        text: r###"Brook stood in the tran-station outside the Medical Center, amazed at the number of people gathered there, waiting for the second coming of Axen Moon. Small pots of flowers lined the entry-way, many sporting handwritten notes. Papers were plastered over

the bulkhead signs, letters, and children's paintings wishing Axen well. A group of people stood watching the safety lock expectantly and talking quietly among themselves.

How things have changed.

How indeed, and though nobody knew it, those changes were all Brook's doing, directly or indirectly. It had been four weeks since the colony relocation, and opening the Medical Center had become a colony-wide priority. The sooner it opened, the sooner

the injured Elder could be brought out of suspended animation and treated for his injuries, and today was that day.

It had

been many months since Axen Moon miraculously arrived back in Eden, unconscious, half-dead, riding in a stolen Scout, with his Savant computer Kraft broadcasting the incriminating news about the mining colony on every available data channel. It had rattl

ed Eden's society to its very foundation and started a wave of change that continued today.

Shortly thereafter the colony had been forced to relocate, and when they'd put it back together, everything had been different.

Someone took his arm gently. He st

arted as he turned and realized it was Echo. She smiled as though nothing had happened between them, and they were two young lovers out for a stroll. "I wondered if I might run into you here. Have you been avoiding me?" She clicked her tongue. "You k

now, I had made arrangements for us to be on the same Evac Transport, but somebody changed those arrangements at the last minute." She locked eyes with him. "Curious thing, don't you think? And then, I just never seem to see you anymore, you don't answ

er my calls, don't do me my little favors. What's happened between us?"

"I've been busy. We're setting up a colony site in record time here."

She glanced at the decorated bulkhead. "Especially the Medical Center I see. Amazing what a little public will can do. I can't say that I'm glad to see the old fossil back, but I can't really complain." A smile crept back onto her lips. "With the new

status quo, the Masters have gone mainstream. More and more people are listening to what we have to say, and th

e authorities don't dare touch us." She chuckled. "As I said, amazing about public will and all." She looked up at him. "Doing anything later, lover?"

He stared straight ahead. "Yes."

She looked down at her feet and shook her head. "Pity. Jacque isn't busy tonight." She swayed from side to side. "Jacque can be a lot of fun."

Brook ignored her, which seemed to make Echo angry. "He killed your friend, you know!"

"Not so loud!" He looked around nervously to see if anyone was listening, but they seemed to be safe. "Look, why don't you and your terrorist buddies just go soak in the mainstream or something?"

She just frowned. "Suit yourself." She turned and started to walk away. "I'll be in my quarters tonight, if you want to call me. But don't wait too long."

d # # #

Brook stepped between the two Volunteer Guards, dressed in their newly issued blue and gold-trim uniforms, and into the private room. One of the guards peered inside and made eye contact with the occupant.

"It's fine; close the door." Axen sat propped up in the med-station bed, an oxygen tube clipped into his nose.

"How do you feel?"

"Not bad for a man who s been shot, blasted, dosed with poisoned air from a damaged spacesuit, frozen, and defrosted. Yourself?"

"They said you'd asked to see me."

"They said you've been checking on my condition every hour for the past two days."

"I have an investment in you."

Axen's eyes narrowed as he stud

ied Brook's face. "You obviously risked your life and status to rescue me, and to liberate Kraft. So why do I feel like we're not exactly old friends here? You got me back as far as Eden, then pulled a disappearing act and made it look like I'd escape

d. Why?"

"It was easier and more forgivable for you to steal a Scout and a Savant than me, especially if it knocked the wind out of the Council. Never mind that it was impossible for you to steal those things, I knew people would believe it of you. The

impossibility only adds to your mystique."

"You made me a hero."

"That was an unintended side effect. Mostly I was covering my own backside."

He grunted. "I might have done the same, if I'd been clever enough to pull it off." He leaned back into his pillow and closed his eyes for a moment, as though he were already tiring. "I asked for you because nobody tells me anything, and there are ques

tions I can't even ask. What the frag happened while I was on ice?"

Brook kept his face a blank mask. "First

we have to settle something. I'm going to ask you a question, and whatever the answer is, I want the truth. If you can't tell the truth, keep your mouth shut and we'll end it right here. How you answer may not change what I do

you're a means to an end now but I have to know before I can work with you."

Axen's brow wrinkled. "Go ahead."

"Did you kill Della Ricca?"

Axen's mouth dropped open. "What? Where did you get that idea?"

"From Echo Van Dozier, initially, but it made a lot of sense. The

window was sabotaged, and you were in the CC repeatedly, including just before it happened. You didn't like my associating with her, and she didn't seem to care much for your hanging around the CC to visit Kraft."

"Poor reasons for killing somebody." He sniffed. "If I were that cold-blooded, Nguyen would have parted company with his head long before he had a chance to blow this planet wide open. Killed her! Great Maker." He sniggered for a moment, then sobered

and looked up at Brook.

"You do believe me, don't you?"

"Yes, I'd come to much the same conclusion, I just didn't want to believe the alternative."

"Which is?"

"That Echo killed Della, directly or otherwise."

"Van Dozier, the baby doctor? First you tell me she's playing amateur detective, then you tell me she's a murderer."

"That baby doctor is the leader of the Masters, the terrorist group. She used me to get to the Gene Bank, then destroyed it, intentionally, I now believe."

"You're sure of all this?"

"She's admitted most of it to my face. She's insane, Axen, I see that now. She believes we have all the genetic material here that humanity will ever need, all us 99 th

percentile humans. Clone us forever, and usher in the golden age."

"I assume you can prove all this. Let's just have her arrested."

Brook pulled a straight-backed chair over next to the bed and sat down with a sigh. "Things have changed while you were out of the picture, Axen. Everything

has changed. When news of the illicit mining camp broke, the Council was disban

ded, several members put under arrest. The chairman was cleared of any involvement. They'd taken advantage of the distractions created by the disaster to run this without the chairman's knowledge. The chairman's forming a tribunal to charge them formal

ly, and the VG are being organized into a real police force."

"This all sounds to the good. So, have the new police arrest her."

"It's not that simple. The existing power structures have been split and fragmented. Some people see the Masters as heroic revolutionaries, visionaries. Echo has gone public, selling her ideas as a kind of immortality."

"People are buying this?"

"Those who haven't bought into the idea of you as a new messiah and maybe some of those too. Enough to make them politically untouchable at the moment. They signed their handiwork when they hijacked the Gene Bank. If they were going to be arrested, it

would have already happened."

"Frag, this is bad." He looked up. "Where's Kraft?"

"Under protective guard in the next room. I don't think they wanted you wearing yourself out talking to the computer." He chuckled. "Kraft is almost as much a hero as you are. The good news is, you're probably politically untouchable too. I don't thi

nk you have to worry about any more assassins. Nobody wants to make you a martyr."

Axen frowned. "How did you know about that?"

"You babbled halfway back to Eden. You'd be surprised what I know." Actually, the attempt on Axen's life was about all he'd heard, but he'd just let Axen stew anyway.

"You think these guards can be trusted?"

"The VG seem to be staying neutral, and no one faction has a clear advantage. The chairman and his supporters are focused on survival and keeping the colony running. You have a lot more power than you did before, but it

s effectively canceled out, just like everyone else's."

"You've seen the Van Dozier woman lately?"

"A few days ago. She was chiding me for hanging around the Med Center. I told her to get lost."

"Did she?"

"She didn't give up, if that's what you mean." He felt his jaw clinch. "I still have feelings for her. I know she's insane, but part of me just wants to fix her. But I'm staying away from her. Don't worry."

Axen grunted. "To the contrary, that's exactly what I don't want you to do."

"What?"

"Be her friend, give her what she wants, within reason, tell her I'm a murderer, whatever it takes."

"Axen, I can't."

"You can, and you will. We can't let this situation continue, and if things are as evenly balanced as you tell me, this is our only hope of gaining an advantage."

Brook said nothing.

"You'll do it."

Reluctantly, Brook nodded. Ask me to saw off my right arm. It would be easier.

d # # #

Brook paused in front of the office door. Dropping in here had seemed like the best thing to do. It would put Echo off-guard, and lacked the intimacy of any of a number of other possibilities. While he hoped to establish some sort of business relations

hip with the Masters, he hoped to keep Echo at arm s length. Anything else would be unbearable.

But now that he stood in front of the door, staring at the fake wood grain and counting the rings, he was having second thoughts. Perhaps I should just go back and tell Axen that I can't pull it off.

But that idea was dashed when the door opened from inside. Echo looked up into his face and smiled. She looked surprised, but not very surprised.

"Brook, I was just thinking about you." She stepped back and beckoned him into the office. "Don't be flattered, I wasn't thinking very hard." She sat on the edge of her desk and gestured Brook toward a visitor's chair.

"So, come crawling back to beg forgiveness, or is this not a social call?"

"It's not funny, Echo. None of this is funny. I don't like being here."

Her smile faded. "Then leave."

"I had a talk with Axen Moon today. I gave him a chance to prove his innocence in the death of Della Ricca."

"And?"

"He had nothing convincing to offer. He pleaded innocence of course, but I don't buy it. That's why I'm here." He studied her face, trying to read the emotion there, but it eluded him. "That's the only

reason I'm here. Axen Moon is too powerful since his return, and I can't stand by and see that continue. On the other hand, there's no direct way I can stop him. My only hope is to ally myself with a force strong enough to counter him."

"Meaning the Masters."

"Maker help me, yes, meaning the Masters."

Her face became very serious. "Brook, you think I'm crazy for my attitudes about the loss of the Gene Bank, but I want to show you something."

She walked over and unlocked a cabinet near the med-station. As she opened the door, wisps of white fog rolled over the lip and faded as they fell toward the

floor. Echo pulled on a pair of heavy gloves and reached inside. She removed a frosty metal cylinder half a meter high and fifteen or so centimeters in diameter. The cylinder was connected by metal-clad hoses and wires to some mechanism hidden in the

cabinet. "I'm not crazy, Brook, I'm calculating. Outside the inner circle of the Masters, you're the only one to ever see this."

"What is it?"

She held it just a little higher and smiled proudly. "It's my own private Gene Bank, culled over a period of

years from the original. While the original had tens of thousands of genotypes, this one holds only a little less than a thousand, hand-picked to be the best of the best. These aren't the 99 th

percentile Brook, they're the 99.9 th percentile, the finest minds and bodies from every significant genetic group that existed on Earth. This is my utopia, not some selfish dream about cloning ourselves."

Brook laughed sarcastically. "But you already have."

Echo just smirked. "Rank has its privileges, Brook, y

ou should know that. Besides, its just a little game I play. You could play it too. Would you like to be cloned, Brook? A little tissue sample, some altered paperwork, replace a vial when nobody is looking, and before you know it, some unsuspecting co

uple gives birth to a little Brook."

Brook's stomach did a flip flop and he felt ill. "No, Echo, I don't want a clone. I just want revenge."

She smiled smugly. "Clones. Revenge. Two things I'm good at."

d # # #

Despite the protests of his doctors, Ax

en wasted no time in getting back on his feet. He was still weak and short of breath, but the thought of Kraft sitting guarded in the next room was a prime motivation. There was much work to be done if the current situation was to be salvaged.

When he finally was able to reach the room where Kraft was stored, he had to again thank Panati for his resourcefulness. Not only was the computer hooked up to the power mains, but Panati had also connected a residential data tap. Seemingly a small thin

g, but Kraft had been trapped behind some kind of heavy security firewall since the Savant had been taken from Axen during the first evacuation.

He sat down next to the computer and put his hand on the cool, smooth upper surface. The first order of business was to find out more about the baby doctor. What had Panati said, something about her bragging that she was a clone? "Kraft, access birth a

nd genetic records for Dr. Echo Van Dozier."

"Access denied. This information is confidential and classified."

"Override."

It took only a few seconds once the command was given. Axen had infinite respect for the computer's abilities

he had long overridden any restrictions in the computer's programming that might have caused it to hesitate in its probings. "Information on screen."

He stared at the document on the screen, then read it again to be sure he had it right. Then he cross-referenced to another birth record. He put a blank data-slip against Kraft's upper surface. "Transfer these two records to the data-slip.

"Done," the computer replied instantly.

Axen took the slip and put it in his shirt pocket. Information was a weapon.

Not everyone appreciated that fact, but Axen did, and he knew he had a weapon against Echo Van Dozier. It was a small weapon, and might only work if delivered by just the right person at just the right time, but it was his now, and he intended to see i

t used.

All he had to do now was figure out a way to get it to Panati."###,
    },
    NovellaChapter {
        id: "s_e09.rtf",
        title: r###"Chapter 9: Treachery: Investigation"###,
        is_eden: true,
        chapter_num: 9,
        text: r###"Brook was getting to be good at stealing vehicles. He was getting a lot of practice lately. He was even getting the hang of the manual controls that all Eden vehicles were equipped with as an emergency measure, but which were otherwise rarely used. The

tiller bounced in his gloved hand, and he applied a little right pressure to avoid a fresh meteor crater.

As the Cargo Truck bounced its way around the back side of the Rare Ore Smelter, where it would be shielded from any curious eyes back in Eden, he wondered again if he should really be going along with this plan. Actually, he had no idea what the plan wa

s, he knew only the explicit instructions Echo had given him, and he'd followed them to the letter.

Axen's clandestine messages had encouraged him to cooperate. Axen seemed to think that the Masters had bigger secrets to hide than he knew, and that this m

ight provide them with new leads. He spotted a suited figure that had to be Echo standing at the building's base in a shadow a small setback, like a cave in its vast silver cliff.

He braked the truck to a sliding stop that threw him forward against his straps. He muttered to himself as he shut down the power system and locked the parking brakes. The hatch slid open immediately. He hadn't bothered to pressurize the cabin when lea

ving the GORF, and Echo climbed up on the broad steps. He could see her smiling behind her visor. "I really didn't think you'd do it. You aren't quite the bad boy you make yourself out to be."

"Maybe I've just got you fooled about the bad boy part. I'm here, aren't I? Besides, you told me you just want to borrow this, though I'd still like to know why you want to borrow the cargo." In the trailer was a battle-damaged Lynx. Its turret destroy

ed during the last skirmish with Plymouth, the rest of the chassis badly damaged, it had been bound for metal recycling when Brook had intercepted it. About the only possible use he could think of for it was as ballast.

Echo shook her head. "I'm not ready to trust you with all our secrets yet. You keep providing things we need, and we'll talk about it, but not today. Just trust me on this one, and I'll be much more inclined to trust you in the future." She shook her

head back toward the power plant. "Out. There's a scooter back in the recess you can use to get back to the colony."

Reluctantly he climbed out as she slid past and into the seat. Her hands ran over the controls with a familiarity he'd never felt when driving. Where had she gotten the chance to practice that? "Looks good. I'm out of here."

He kept his perch on the steps. "Wait, when will you be back?"

She considered his question for a moment before answering. "About twenty hours for my business, plus travel time, call it thirty-five hours. Forty tops. You said nobody will notice the truck is missing."

"Operations thinks it s in the Garage, the Garage thinks

it's part of the starship salvage convoy, the truck that's actually in the convoy was rotated out of ore hauling, and when you bring this truck back it will replace it. As long as you beat the convoy back, there won't be a problem, but I can't hide it fo

rever. Same with that junker in the bed. Since the mine colony scandal, there are much closer tabs kept on recycled items and total output."

"You'll get your salvage back, pound for pound. Now, the sooner I roll, the sooner I'm back. Off."

He stepped

down and away from the truck, and watched as it accelerated smoothly, turned, and using the power plant as cover headed out into the foothills, into thousands of miles of nothing as far as Brook knew. But somewhere out there, seven and a half hours away,

was something, a destination. That would be what Axen was trying to uncover, but not if Brook could uncover it first.

d # # #

Axen leaned back in his chair and contemplated the array of pictures, lists, and documents that Kraft was displaying on his apart

ment's big EnterCom screen. His goal was to identify the Masters and their resources, not at all an easy task. Although the group had technically gone public, only Echo, of all the core members, had publicly identified herself. The rest were hidden, an

d their anonymity made them especially dangerous.

There were others who identified themselves as part of the group, but they were newcomers and hangers-on, seduced by Echo Van Dozier's rhetoric, and who knew little of the group s actual agenda or activitie

s. With the exception of Van Dozier, if they loudly proclaimed their membership, they weren't of interest to Axen.

That left Echo, the three goons she'd brought with her when she'd attempted to recruit Panati, and a few more that he'd identified since. For these few Axen had names, pictures, information. The rest, however many there might be, were a mystery, and a d

ifficult mystery at that.

But Axen had been luckier in tracking down deceased members. Many of those who had been at the mining outpost had been sent there because of probable association with the Masters, Vox Borges being one of them. Axen still wasn't sure if Borges

attempt to kill him had been initiated by the Masters, or if Borges had struck a deal with the now-deposed Council. There

had been others, some of whom had been identified as having died in accidents, some Axen had ruled out as active members for one reason or another, and three who were simply missing.

These three were also identified as having been killed in an accident, but unlike the others, no remains had been identified. Curiously, despite their new-found power, the Masters had not protested or called for further investigation.

Axen walked up to the screen, took out a data pen and wrote next to the three names, "DEAD?" He considered this for a moment, then wrote above the names, "LOOSE CANNONS." But if these three weren't dead, where were they?

From there, Axen had set Kraft investigating the dispensation of everything from the mining outpost. Again, there were cur

ious holes. Most of its resources, units, and buildings were listed as "abandoned in place," unsalvageable. Yet, a count taken from satellite photos didn't jibe with the inventory. A handful of items were unaccounted for, including a Robo-Miner, a ConV

ec, a Command Center kit, and a Laser Guard Post. A Lynx totaled during the Plymouth raid on the outpost was also unaccounted for, though its hulk should still be on the battlefield where it had been disabled.

He looked at the short row of pictures at the top of the display. Thanks to Panati, he knew at least these few for sure. "Kraft, run up a profile on all these, up to and including their current work assignments and whereabouts."

"Certainly, Axen." There was a pause. "Preliminary report is on your ClipCom. I am attempting to create a deductive profile from less direct sources."

Kraft, I taught you well. It's good to have you back. He scanned the list quickly. Gi Atolo was a scientist currently working on

agricultural research. Sharon McComb worked in a Garage. The others were an unexceptional lot, working common jobs in less-than-key locations. The one oddity was Jacque Barre, who was on disability leave from his job in the Vehicle Factory. Panati ha

dn't mentioned any obvious infirmity.

Axen tapped the name on his ClipCom. "Kraft, give me more on this."

"He is listed as having been injured in a robotic malfunction three weeks ago, confined to bed. I find a listed emergency response of that description on that date, but indirect information causes me to suspect that record."

"Is he in his quarters?"

"Probable."

"On what do you base that?"

"Service logs show appropriate drains on life support. There are ongoing records of communications activity, computer use, changes in electrical power demand, all consistent with occupation. His ration points have been used to purchase meals in his Resi

dence."

Axen chewed a fingertip nervously. He could think of ways to fake all those things. "What about his luxury ration points?"

"They have been used also."

"Where?"

"Three different Recreation Facilities, the Consumer Goods Factory, several public eating places around the colony."

Axen smiled. They'd done a good job, but somebody had gotten greedy. If Barre wasn't around to use his luxury points, why should they go to waste? But an invalid wouldn't be spending them all over the station.

So, where was Barre? With all the other missing items? With the other Masters, supposedly dead? Possibly, but it was too soon to jump to such conclusions.

His ClipCom chimed softly, and a message icon flashed on the upper corner of the screen. "Answer, visual," he said. The face that appeared on his screen was Zek Autzen, son of one of his fellow Elders, and a member of the new Senate. Zek nodded respect

fully, though the two of them were hardly strangers. "Elder, the chairman wishes to see you in a personal meeting. There's a favor to be asked."

Axen raised an eyebrow. This was unexpected. "Care to save me the suspense and tell me what this is about, Zek?"

"I don't know exactly, Elder. It's my understanding that they want you to act as some kind of diplomat."

d # # #

Brook sat in the now familiar alcove under the edge of the Rare Ore Smelter, watching waves of dust blow past the opening. He was sitting on bare ground, and despite the suit's insulation and heaters, his butt was getting cold. He looked at the chrono d

isplay on his helmet's heads-up display. Where was she? He leaned forward and watched the thin, high clouds moving across the sky at a frightful pace.

New Terra's weather had become increasingly erratic, in part because of the thicker atmosphere and the change in the gasses making it up, in part because of the volcanic activity in the Blight-infected areas. The thin, dry air had begun to spring new sur

prises on Eden s scientists. Dry lightning storms, apparently caused by static electricity, had begun to appear on the outskirts of the colony, as well as dangerous funnel clouds the scientists had dubbed "vortexes."

Every time the scientists came up with a weather model that worked, the environment changed enough to render it invalid, making their forecasts almost useless. There were no serious storms forecast for today, but his gut told him otherwise. He didn't wa

nt to be caught alone on the surface with only a suit and a scooter.

Something in the distant windblown dust caught his eye, nothing more than a slight thickening in the cloud. He watched carefully. Yes. Dust kicked up by a vehicle.

He stood, brushed hopefully at the dust covering his suit, and finally settled on getting his visor relatively clear. He stepped from his shelter, and actually felt

the wind tugging at him. At this pressure, that meant the wind had to be moving at 120 kilometers per hour or better.

As he watched the truck getting closer, he saw another thickening in the dust, and at first thought it was a second vehicle. Then the funnel pulled reddish soil up its otherwise invisible length, like a child sucking orange juice through a straw.

Vortex!

He eyed the truck warily, and wondered if Echo had seen the funnel. The vortex was perhaps seven hundred meters behind her, and might be hidden from her view by the cargo box. If she checked the rear-view cameras she'd see it, but there was little reaso

n to do so on the open desert.

As he watched the seconds tick by in the corner of his eye, he tried to judge the movements of both the truck and the vortex. If it didn't speed up in its lazy, w

eaving dance across the desert, she'd beat it here easily. He could get in the cab, to heck with the scooter, and they could make a run for the safety of the nearest Garage.

Then another discomforting thought occurred to him. What if she saw the vortex and didn't

stop? It would be quicker to make a straight run for the Garage, and Brook had no way of judging if she cared at all for his life, or if she would consider him valuable enough to take a risk in rescuing.

Quickly he surveyed his surroundings. There was a service access door a few meters up. It would be locked, but Brook had a hacked keycard that might contain an override that would work.

He watched the truck grow closer, as did the vortex, which had grown dark and tall, the soft fuzzy column of its funnel tightening down into a smooth, thin, writhing snake. Brook knew what that meant, that a faster inner column of spinning air was formin

g, a corkscrew of wind so fast it had never been measured, such a focused stream of destructive force that scientists jokingly called it an "air laser."

The truck at first seemed to be taking a wide arc around the smelter, but then it turned toward him, seemingly in no great hurry. The vortex was getting closer, its funnel whipping back and forth now, like the tail of some angry beast.

Hurry. She couldn't have spotted the vortex; she was loafing along at nowhere near the truck's top speed. Frag.

The vortex was going to overrun her before she got here. Quickly he ran back to the scooter and hopped on. He gunned the throttle and the fat tires kicked up a rooster-tail of dust that the wind sucked away greedily.

He had to make it to her first. He ran the bike flat-out, the tires slipping from side to side in the loose sand, gusts of wind trying to knock him off balance. The vortex loomed over the truck like a giant about to stomp on an ant.

He laid the bike over on its side without slowing down, sliding to a stop in the truck's path, praying that a sharp rock didn't split open his suit. The truck veered t

o avoid him, slowing enough that he was able to jump onto the step and grab hold of the door grab-handles as it opened. He threw himself inside.

Echo glanced over. "Brook, are you out of your mind?"

"Turn right, hard. Vortex." She looked shocked, but she didn't hesitate, and the truck slid around in a tight turn.

Brook glanced at the rear-view screen and saw his scooter, half flattened by one of the truck's tires, nearly disappear in the clouds of dust at the funnel's base, then reappear as an explos

ion of fragments that went in all directions, as though it had been blown apart with a grenade. "Faster!"

But it was too late. The truck, already off balance from the tight turn, was hit by the vortex, which tried unsuccessfully to lift it into the sky. Instead, the truck began to roll slowly, first up onto its side, then completely over.

Brook, not being strapped in, landed in a heap on the cab ceiling. The whole cab bucked and shuddered as though it were being torn from the truck, then dropped to th

e ground with a crash. Brook rolled over onto his belly and looked out the window. The huge base of the funnel had moved past them, but then it paused in its drunkard's walk and started moving back toward them.

He reached back and pulled the emergency release on Echo's harness, catching her as she fell free and immediately tugging her out the door. The funnel was coming at them fast now. If it had hit the cab with them still in it, he was certain they'd have b

een killed. Of course, if it caught them out in the open, they'd be just as dead.

Brook glanced at the overturned cargo box. A small depression in the ground had created an opening under its lip that might be just large enough to crawl into. He dragged the dazed Echo toward it. It was too loud to hear her or even tell if their radi

os were still working. He shoved her toward the opening, and she got the idea. He crouched next to her and helped push her through the tight space. The vortex was almost on them now. Brook climbed into the openin

g, digging his way frantically into the dark space with hands and feet.

He slipped into the tight space between the cargo box and its combat vehicle cargo, but Echo was nowhere to be seen. Then her helmet light flashed from the other end of the box, and he saw her waving him frantically that way. He scrambled on hands and k

nees toward her, and she motioned him toward an engine cover on the back of the combat unit. Of course. The Lynx might be heavy enough to resist the vortex.

The cover hung loosely

by one corner, and there was just enough room in the motor bay for the two of them to squeeze inside. He jammed his back against a bulkhead and braced with his feet. He glanced over at Echo. If he'd wanted to be rid of her, he'd missed a perfect chance

, and now his attempted rescue might end up costing him his own life.

The roaring outside reached a crescendo, and their dark hiding place was suddenly flooded with light and dust as the truck bed lifted away. The Lynx shook, seemed to lift slightly in the air, made a 180 degree turn, and then dropped back to the dirt.

The roar faded, and Brook caught a glimpse of the vortex withdrawing into the sky. Suddenly claustrophobic, he leaped out of the hatch and ran a few meters before falling down in the dust. Echo staggered out, walked over, and plopped down beside him. S

he laughed. "Guess this truck is off the books for good, eh?"

He

laughed too, more from relieved tension than anything else. Then he noticed something about the Lynx. It was hard to be certain with it sitting upside down, but he was relatively sure he was right. It was the same turret that had left Eden hours before

, but the chassis was different. Oh, it was the same model, and it was combat damaged, but the damage was much more extensive, and in different places. Somebody had swapped the trashed turret onto an even more trashed body and shipped it back.

Why?

He looked over at Echo, who made eye contact, a big smile on her face. His heart sank. He'd just saved her life. Would he be able to do whatever was necessary when the time came?

d # # #

Axen sat down at one end of the empty Senate table facing the big screen set up at the other end. For the first time in decades, he would have a legitimate, legal communication with Plymouth.

Give it to the chairman, when it comes to survival issues, the bases are covered. In secret, even as the shooting war was heating u

p, lines of communication had been opened with Plymouth, and the stage set for negotiations, not to end the fight, but to ensure the safety and survival of Plymouth's human Gene Bank.

Axen had been reluctant to step into the role of negotiator, but the chairman had insisted that he was especially qualified, and had granted him significant powers. It was just possible that he might be able to accomplish something here today.

He'd spent the last ten hours researching the situation, developing various proposed agreements that would allow for the transfer of the Gene Bank if when

Eden completed its starship. He felt confident that he was ready to deal with any situation.

Then the screen lit, and Emma's face looked back at him. Her expression was blank. This was obviously less of a surprise for her than him. "Axen," she said, "it's been a long time."

Which, officially, it had been. Even unofficially it had been long enough. "Emma. They hadn't told me who my counterpart in Plymouth would be. This is a surprise."

"I see that, and yet, somehow I think I know exactly what you want."

That's playing it a little close to the surface, Emma. We have to be discreet, for both our sakes.

"We're looking for a guarantee that the Gene Bank will be transferred to us if it's clear that Plymouth will not finish its starship in time. In return, we will guarantee the safety of the Gene Bank, if it is placed in a designated and marked non-strat

egic structure, and that Eden will be allowed to depart unmolested if its ship is finished first."

She nodded while keeping eye contact. "We're prepared to discuss it, Axen, though you know some things are non-negotiable." Something on her desktop caught her eye. She touched an icon, and a document appeared. She read it quickly, and her jaw dropped

, then clinched shut as she looked up at him, eyes burning like coals. She looked at someone off-camera and said, "Bring him in."

She turned back to him, looking no less angry. "How could you? What were you trying to accomplish? Steal it, or just locate it for future reference, in case we didn't agree to paint 'steal our Gene Bank' on the outside of the building?"

He shook his head in puzzlement. "Emma, I have no idea what you mean. What's happened?"

"I hope you're telling the truth, Axen, but it makes no difference in the outcome of our talks today. As of now, they're over."

She glanced back as two uniformed men dragged a third man into camera range. The man's wrists were cuffed behind him, and the guards stood on either side holding him up. A large bruise stained one side of his face.

"We found him in a restricted area adjacent to where the Gene Bank is stored. Do you know this man, Axen?"

Unfortunately, he did. The man on screen was Jacque Barre."###,
    },
    NovellaChapter {
        id: "s_e10.rtf",
        title: r###"Chapter 10: Stalemate: Raid"###,
        is_eden: true,
        chapter_num: 10,
        text: r###"Retribution had come swift and hard for the actions of Jacque Barre, "Eden Spy." Under cover of darkness, a Plymouth convoy had sliced through Eden's defenses, ignored choice strategic targets, and made a lightning raid on the labs researching starship t

echnology. An attempt to remove salvage from the original starship, Conestoga

, had been repelled thanks to courageous action by the Volunteer Guard, but three scientists were missing and presumed captured. Among these was Eldon Jensen.

Axen especially regretted Jensen

s loss. Failure was always easier to take when it didn't have a face. Of course there was nothing he could do, once Barre caused the talks to collapse. Emma, or at least the people she spoke for, simply didn't trust any

thing Eden had to say, anything they had to offer. When the communications channel went dead one afternoon after a particularly heated session, it was no surprise.

The raid nine hours later was.

As Axen sat in front of one of Residence Three's gallery windows and stared out at the large hole blasted in the lab's side, still unrepaired, he hoped Plymouth had gotten something useful from the three captives, because the pendulum was swinging Plymout

h's way again, and it was bound to hit hard.

Alarmed by the ease with which the raid had penetrated their defenses, Eden had been given a new priority, an

d its splintered political factions had aligned behind one thing and one thing only: a massive military buildup. They would have their scientists back, and the Gene Bank too.

Axen slumped, chin propped in his hand. If this kept up, neither side would win. Too many resources would be squandered on weapons, too much destroyed in battle. Neither starship would be built, and humanity would die. He had to keep the lid on this w

ar, keep things from falling completely apart before the most vital work for survival had been completed.

"Elder, you look like someone shot your dog."

He looked up as Zek Autzen stopped in front of him and handed him a ClipCom. "What does that mean, the dog thing?"

"I don't know. Saw it in an old vid once, and you had the same look on your face as the guy in the vid. They never did show what a dog was, though."

Axen tapped the ClipCom with his finger. "You read it?"

Zek nodded. "Me and the chairman. We agree with you that it's premature to take it to the whole Senate. It's shocking, if true."

"It's true all right. I'd stake my life on it."

Zek sighed. "Even if it is, Elder, I don't know what we can do about it. This could tear the Senate apart if we reveal it to them, destroy the uneasy truce we currently operate under. The Masters would deny everything, everyone would choose up sides, a

nd meanwhile, it could force the Masters to play out their hand."

"You'll have proof."

"That some units, parts, and supplies may be unaccounted for? That some people associated with

the Masters didn't come back from the mining camp? That one man is missing? You can make a lot with that, but not a case. If what you tell me is true, we need a location.

So far, the satellite has turned up nothing, and even getting the resources to look is politically very difficult right now."

"They'd hardly leave it all lying around in plain sight. They don't want either us or Plymouth to find it."

"I need more, Elder. Give me something to work with and we can at least fight it out in the Senate.

" He stood and started to walk away. Then he stopped and half-turned back. "Assuming you're right about what they've got, and what they're going to do with it, do you think they have a chance?"

Axen shrugged. "It s a lot easier to destroy something than to steal it. Even if they fail, they'll stir up Plymouth so bad that we'll have no hope of ever getting the Gene Bank, by any means. Either way, they win."

Zek sighed. "And humanity loses."

d # # #

Brook impulsively stuck his hand in his pocket and felt the small transmitter card. Axen had assured him that it could be attached anywhere on any vehicle and still be effective, and that it would be extremely difficult to detect, even if someone were lo

oking for it. It transmitted only in microsecond pulses at random intervals on a system of rotating frequencies normally used by suit radios, telemetry systems, vehicles, mining beacons, and other units one would expect to find active around the colony a

nd even out in open country.

None of which made him feel any more comfortable about it. He didn't know when he would have to use it, or under what circumstances, and if one of the Masters caught him with it on his person, well, he didn't know what would happen. On the other hand, h

e couldn't very well go out without it, especially when he suspected the reason for Echo's requested meeting at the Agridome was to borrow another vehicle.

At least, as he waited in the darkened structure looking up at the sharp

pinpoints of stars in the sky, he hoped so. It had been nearly a month since she'd made such a request of him, and Brook was beginning to wonder if Axen's theories, as logical as they seemed, were totally off base. Part of him still wanted to believe t

hat Echo wasn't the monster she seemed to be, that it was all some vast misunderstanding, or that she was, at least in some way, redeemable.

He heard a crunch in the corn field behind him. It would be impossible to come that way without destroying plants.

That seemed irresponsible and out of character, even for Echo. He turned and peered back into the darkness. He could see the cornfield silhouetted against the moonlight coming through the far side of the dome, but no sign of a person. "Echo?"

More rustling, closer. This wasn't right. It wasn't right at all. "Echo, is that you?"

More rustling. "Whoever you are, say something." He hesitated. No answer. "I have a gun." Which was a lie, of course. He'd considered trying to black-market a side ar

m for his personal use. He was even pretty sure that Axen had one or two squirreled away somewhere that he might have been talked into parting with. But the dangers and disadvantages had always seemed to outweigh any benefit. Right now, those arguments

seemed to have lost some of their weight.

His eyes strained into the darkness, looking for some movement, some identifiable outline. Then in a flash it was there, charging through the nearest row of stalks, crashing into him, pinning him back against the

clearplex. "Where's the gun, Brookie-boy?"

Though he couldn't see the man's face, could barely see at all with the thick forearm pinned against his throat, Brook recognized the voice as Gi, one of Echo's more constant associates. He felt the man patting him down with his free hand, and flinched a

s he touched the pocket with the transmitter in it. It was difficult to breathe, much less talk. "Just a bluff, Gi. Didn't know who you were."

Gi seemed satisfied and lessened the pressure, though he kept Brook against the window.

Brook decided to take the initiative. "Where's Echo? Why the rough stuff?"

"Somewhere safe. Things have gotten pretty hot around here for us, and Echo decided it was time for a security review. Make sure you weren't followed, or any other such frag."

"I'm clean, Gi, you can see that."

Gi nodded. "Yeah, seems like it." He took half a step back, straightened the front of Brook's shirt, then stepped back again. "No hard feelings, Brookie-boy."

Brook restraightened the shirt, and decided that the garment was never going to hang right again. "Yeah, if you'll just stop calling me 'Brookie-boy.' Name s Brook, and I like it."

"Suit yourself. I got a message for you from Echo."

"I got a message for her. Next time, bring her message in person."

Gi frowned, his face a blue mask barely visible in the moonlight. "Some advice. You be careful who you order around. She may have let you put the make on her, but she's a powerful lady, a dangerous lady, and she'll be more so before she's through."

"But until then, she still needs favors. Cut the gorf and tell me what it is this time."

"She needs a ride starting in thirty hours, something with armor would be good, for a one-way haul."

"What?"

"This one isn't coming back, or at least, not soon enough to make a difference to you. Color it gone."

"I can't do that. Tell her I can get her a Scout for a day, maybe two, but that's it."

"Look, I don't negotiate here, I just deliver the message. She says come through, I suggest you come through." He smiled, and it wasn't pleasant. "Otherwise, it could be bad."

Gi turned and walked away back through the cornfield, leaving Brook to curse the night.

d # # #

Axen sidled into the back row of the dim gallery that overlooked the Ping-Pong table and made some show of watching the tournament going on there. He glanced over at Zek, who was munching from a bag of some orange snack-food.

Zek saw Axen eyeing the bag and tilted it toward him. "Carrot puffs. Want one?"

"Not if a meteor just hit the last Agridome on New Terra."

Zek shrugged. "You're the one who keeps insisting on these secretive meetings, Elder. You could just come to my office like anyone else."

"That would be too," he searched for the right word, " obvious . The less we're seen together, the better, especially considering what I have in mind."

"You've got a location, then?"

"I expect to shortly, but then, I don't think we can afford to wait on that. I want all the pieces in place when the time comes."

"Time comes for what? I told you, Axen, even if you can prove the existence of this 'resistance base,' even if you can pinpoint it for me, there isn't much we can do. Even with the Masters

spy blown, we can't just unilaterally move against them."

Axen nodded. "No, you can't, and if you reveal the existence of this thing in an open Senate meeting, it will change from a treasonous act into a subject of debate. But right now,

it doesn't exist, and that can work to our advantage." He waved a hand at the players at the table below, the ball flying back and forth with blinding speed. "That's why we're here rather than your office. I'm convinced that the Masters are going to u

se their base to stage a preemptive strike against the Plymouth Gene Bank. You can't stop it, but maybe I can. I'm going to offer you an out."

"I don't understand."

"We can't destroy the base if it doesn't exist. How could the Masters raise an official protest if the base doesn't exist? They can't."

"Destroy their base?" He laughed. "With what, Elder? Are you going to go club them to death? I certainly can't allocate forces to go after them. If I could, I wouldn't need you."

"But you're on the weapons development committee, Zek. You've been working around the clock to develop the new turret for the Tiger chassis."

"That's classified, and I know better than to ask how you found out. But what's your point?"

"Back on Earth, Zek, they had a very useful term. It was called 'plausible deniability.' Point being, a vehicle under manual control normally will leave no record of its movements, and new weapons, well, new weapons need testing."

d # # #

Brook had arranged to have the Scout left outside the Structure Factory's far airlock. He'd left the arranged door unlocked for Echo, and hoped this time she actually showed, not one of her goons. He leaned on a railing trying to look casual, and watche

d the unoccupied assembly machinery below running through a self-repair and maintenance cycle. Frag if he was going to let Gi rattle him.

He heard the sound of boots on metal grating and glanced up. Echo was walking purposefully toward him up the catwalk. She stepped up next to him and adopted an identical pose leaning on the rail. "Floor show's kind of dull tonight. You've got the item

?"

"It wasn't easy."

"I didn't ask for it if it was easy, Brook, I just asked for it. I need it. There are some very important things going on, and I need to be able to depend on you."

He hung his head. "Yeah."

"Then you got the vehicle, no return?"

"I have a Scout waiting outside the emergency lock at the bottom of the stairs." He hesitated before making the lie. "No return."

She was silent for a time, just watching the preening of machinery on the factory floor below. "I can tell when you're not being truthful with me, Brook. If that Scout isn't back in two days, you're going to be in a lot of trouble."

He hoped she wasn't as perceptive as she pretended. "Okay, true."

"I'm not without my own resources, Brook, and I'm not above checking on you. Now, want to tell me why?"

He squirmed. "You said you needed it badly. I figured I could come up with a cover story by the time they noticed it was missing."

"Brook, I'm going to assume that was a noble gesture on your part, but it's also a stupid one. Drawing attention to yourself, or to us, doesn't serve our mutual needs. I need this vehic

le, but not at the cost of ruining our plans." She turned and studied him carefully. "I think I have a solution, Brook. We've put you through a lot lately, and you've proven yourself reliable

to a fault. I'll take the vehicle, but you're coming with me."

He stared at her blankly.

"There's something I need to show you, something I owe you a look at, since you helped to make it possible. And that way, you can drive the vehicle back in plenty of time."

"What about you?"

She just smiled. "I'll have my own ride. You'll see."

This wasn't what he'd planned, not at all. "I'll need to go back to my quarters, arrange some off-time."

She waved her hand. "Done. All taken care of." She took his hand and led him toward the steps down to the lock. "This is an exciting day, Brook. You're going to see the Masters secret army."

d # # #

Axen looked unhappily at the irregularly dotted line on his ClipCom. The signal from the transmitter was coming in loud and clear. The problem was, he couldn't get in touch with Brook to find out exactly who or what he was tracking.

He glanced around the Garage and watched the last worker go on break precisely as scheduled. Three test model Tigers were lined up in the first three bays. He sealed the visor of his spacesuit, slipped his keycard out of a front pocket, and walked towar

d the first Tiger.

The hatch opened immediately when he ran his key over it, and he lowered his feet into the opening and then slipped inside. The short, narrow space pushed his knees up and in, and barely gave him room to pull his elbows in.

He slid down a little farther into a semi-reclining position, giving just enough room to pull the hatch shut over his head. The emergency operations console swung down in front of him, and the screens and pads lit, giving him enough light to look around

and examine the bare-metal coffin he'd just climbed into. The compartment was designed for functionality, not comfort, and it certainly had never been intended for cross-country trips. Unlike most vehicl

es, it lacked even an emergency life-support system, and so he'd have to spend the entire trip sealed in his suit.

He activated the manual controls, and directed the Tiger out of the Garage. Fortunately, the on-board Noesis computer could handle the precise steering. All he had to do was use the console to tell the vehicle where he wanted it to go.

It rumbled to life around him and began immediately to move. It rolled out of the bay, turned on its axis, moved

a few meters, then turned again toward the vehicle lock. He glanced down at the ClipCom balanced on his leg and set a route to follow at a discrete distance. He wondered again where Brook was, but it was too late to worry about that now. He'd soon be l

eaving Brook far behind.

d # # #

Through the Scout's windows he could watch the southerly surface wind kicking up a layer of dust that almost concealed the ground. He flexed his fingers apprehensively and wondered where Axen was.

"Look," said Echo, pointing off to their left, "we have an escort."

The dust was gusting down the slope in that direction, and at first he didn't see it. Then he started to pick up a few edges and managed to build up a picture in his mind. He blinked to be sure his eyes weren't fooling him. It was an armored combat veh

icle, a Laser Lynx if he wasn't mistaken. In place of the normal Eden colors and bare metal, it had been painted in a mottled gray and orange camouflage pattern, making it very difficult to see.

He looked Echo's way. "Yours, I assume?"

She nodded. "Cobbled together from bits of the wreck you loaned us and a second unit we recovered from the battlefield near the old mining outpost. It's not our only combat-capable vehicle, but it's our best."

"I'm impressed, that you have it anyway, but I don't know what you expect to do with it. It's certainly no match for Eden's forces, or Plymouth's either, if that's what you have in mind."

"Agreed, if we were planning a full-scale attack to cause extensive damage. But a well-planned, well-targeted mission to destroy a single item might have a chance." She pointed out the front. "Look, there."

Again, he didn't see anything at first. Then he made out irregular, shifting shapes, like rippling hills. It took him a while to realize that he was looking at a huge roof made out of camouflage netting. He could just make out a row of parked vehicles

and a single structure, a Command Center, carefully painted in camouflage like the Lynx.

"We're very proud of the netting," she said. "It reflects radar just like open ground, and has active thermal compensation to fool infrared. Believe it or not, the netting was tougher to come by than the Command Center."

She leaned down to look up at the CC as they rolled around it. "It's stripped inside, but all we really needed was a building with independent power generation and life support. It's much better than our first base, which was a played-out Rare Ore Mine.

That gave us a place to hide our people and collect materials. Then, during the last relocation, we used the confusion to cover our own construction project. By the time Eden was back up and running, so were we."

As they passed the row of vehicles, Brook could see they had all been modified, crudely, and probably at a huge cost in labor. Weapons and armor plate had been welded on all over them. Rifles, along with their power supplies, salvaged from Plymouth Scor

pions, seemed to be a favorite improvised weapons upgrade, though a ConVec had what appeared to be part of a Guard Post turret grafted onto its upper deck.

"We're ready to go, Brook. Today, or tonight, rather. We'll move out under cover of darkness, hit them from the less defended side, the one away from Eden, use the less capable vehicles as decoys, and ram the Lynx right down their throat for a surgical

strike on their Gene Bank."

"Echo, this is crazy. You've convinced most people that the destruction of Eden's Gene Bank was an accident, but you can't get away with this. You'll lose all your support."

She

grinned and shook her head. "We'll ride back to a hero's welcome. The people don't like the fighting, or hadn't you noticed that? We'll have saved them from a disastrous and pointless war to steal the Gene Bank. Our vision of the future will be assure

d, and we can resume moving forward rather than sideways."

They were approaching the lock when the radio crackled. "Watch to command, we have an incoming, one kilometer out moving along the base of the east valley wall."

Echo looked suspiciously at Brook, but said nothing to him. She switched the Scout's transmitter to its lowest power setting and set a scramble. "How many, watch?"

"Just one, but it looks like a heavy combat unit."

"Frag. Command to nest, the alarm is sounded. Deal with the incoming, then the operation begins, now ."

Echo swung the Scout completely around the CC and stopped just under the netting. She watched as a small swarm of suited figures emerged from the CC and climbed into the various vehicles.

He had to talk Echo out of this before they both got killed. "This won't work, Echo." Oh, that was convincing.

"It's a Tiger, but it s only one Tiger. We outnumber it, and we know the terrain. Whoever is running it will have no idea what the capabilities of most of our units are either. No, I think we'll win easily."

He watched as the various units, led by the Lynx, rolled out. The Scout took up a distant rear position, having no armor and no combat capability. Brook didn't think that would prevent

Axen from blasting them though, unless of course, Echo's assessment of the situation was accurate, and he didn't want to think about that.

d # # #

Axen watched the approaching blips on his targeting scanner. Too many. There was only supposed to be the one Lynx. Were they planning to attack him with unarmed vehicles? Ramming? It was unthinkable.

He swung the Tiger out onto the valley floor to give himself more room to maneuver and less chance of being boxed in. Armed or not, some of these vehicles we

re probably twice as fast as he was. A Laser cut through the blowing dust and an explosive bang told him he'd been hit. The dust would reduce the effectiveness of Lasers, but he had no idea what it would do to his unproven weapon system.

Time to find out. He powered up the turret and activated the path projectors. Behind him, through the metal of the hull, he could hear the massive volt-sinks powering up.

d # # #

Brook, trying not to be obvious about it, scanned the little cab for a weapon. He eyed the emergency hand-pump for the door opener. The handle looked removable. Slowly he reached over to pull the pin that held it in place.

The radio chattered with traffic now.

"Squad Three, do you recognize that turret?"

"Negative. It's putting out some kind of beam though; it just gave me a little bump as it tracked past me."

Echo smiled as she heard this. "It must be one of the new prototype weapons. That's how they got it out of Eden without drawing attention. This is being passed off as a test. W

ell, it looks like their super-weapon has a few bugs to work out." Her hand tightened on the tiller. "Too bad there isn't time."

Gently Brook pulled the handle free of the pump.

d # # #

Suddenly, Lasers were coming from everywhere. These weren't combat vehicles.

Well, never underestimate the human ability to improvise. The Masters had been rewriting the rules from the beginning. He felt stupid not to have anticipated it here as well.

Individually, the hits were nothing, but collectively, they were taking him apart, and he couldn't get away. He looked for cover, but the flat valley floor offered nothing.

The ConVec rolled in front of the turret. Moment of truth.

The path projector's invisible beams swept along its flank and locked on. They'd barely feel those, but they were only to provide a path of least resistance for the electrical discharge that followed. Time to let loose Thor's Hammer.

Fire!

Lightning arced away from the turret, danced around the vehicle for a moment, then faded. The turret's second weapon fired. Again, it narrowly missed the target, then flicked into contact, and sliced the side of the ConVec open like a ripe melon.

Yes!

d # # #

"Squad Two, I just got hit by lightning! Did you see that?"

"Negative, negative, sky is still clear."

"Squad One, pulling back."

Brook touched the bloody lump behind Echo's ear. She was still breathing. He pushed her aside so he could take the tiller. Things were looking better for Axen, and worse for him. He had to find some way of letting Axen know he was in the Scout, and ge

t clear without the others giving chase.

"Squad One, it's a new weapon. Looks like he's having targeting problems though. Stay close and move fast, we'll wear him down."

"Roger."

In a few seconds, Axen would be surrounded again. If he was going to get his attention, it would have to be now. He slid the speed control forward and the Scout surged. He turned and headed right down Axen's throat.

d # # #

Axen looked at the Scout that was coming toward him in a full charge, lights blinking. It didn't look armed or, for that matter, modified in any way. Suicide attack? But it could well be the Scout that Van Dozier left Eden in hours before. This could

be an excellent opportunity to take her out of the picture with no political repercussions, maybe the only one he'd ever have. He locked the trackers and the turret began to spin.

Where is Brook?

The question popped into his head out of nowhere, but it made his finger pause over the fire control. He could have gone with Van Dozier, maybe even been kidnapped. The lights on the Scout flashed, and he wished that he and Brook had come up with some s

ort of code.

Oh well, too late now, and he had only one choice really. He made a final, manual adjustment of the targeting and fired.

d # # #

The flash nearly

blinded Brook, and he flew back in his seat, throwing his arm over his eyes. Even with them closed, the blue streak of electricity was burned into his retina, and he blinked to clear his vision. The Scout was still moving, and seemed unharmed.

Targeting problem, he thought. I can use this.

He keyed the radio. "This is uh command passenger. We were hit. The commander is hurt bad. I'm going to make a run and try to get her to a hospital."

"This is Squad Two. Negative, we have medical facilities at the base."

"She's hurt bad. She needs a hospital. Out."

He flipped off the transmitter, gunned the speed control, and blew through their lines and past the Tiger, which now had plenty of other problems to deal with.

Then a voice from the radio said, "Squad Three, something is wrong here. All units, your new priority is the commander s Scout. All units pursue and capture.

Oh, wonderful. They're all after me, and even if Axen has figured things out, the Tiger will be too slow to catch up.

d # # #

The turret was in constant motion, firing as often as the volt-sinks could cycle. The targeting systems were self-correcting, learning as they practiced, and it was only a matter of time before he was hitting more than not. The tiny compartment began to

heat up, and even through his gloves the back wall was hot to the touch. His suit air-conditioner labored under the strain.

Then he watched as the Scout roared past, and hoped that he was right. When he saw the other vehicles withdraw and give pursuit, he knew he was. Time to close the trap.

He had the ClipCom configured as a secondary control console. He tapped the icon that would start Tiger Two and Tiger Three out of their hiding places up on the valley rim and down to meet the Masters

convoy. That should slow them enough for his Tiger to close in on their flank, and then they'd be easily sliced to ribbons. There'd be time to blow up the base later.

d # # #

Brook steered the Scout on a zigzag course down the valley, dodging Lasers all the way. He didn't see the Tigers until they were almost on him. He felt like a fly about to be clapped between a pair of giant hands. Then the Tigers turned and disappeared

behind him.

He glanced at the rear-view screen. The convoy was in chaos, boxed in, lightning flashing across the valley floor like a carpet of blue fire. Once he reached a safe distance, he slowed, turned, and stopped to watch the show.

Somebody else was watching it too. "I'm going to kill you."

Echo sat motionless, inert, in the other seat, but she looked at him through narrow, bloodshot eyes, projecting a malevolence he had never imagined. "I will kill you, and I will pull every gene sample ever taken from you and I will personally burn them,

so there will never, never, be another you."

Brook licked his dry lips and turned back to the battle. Axen was mopping up, but this was far from over. He thought of the data-slip Axen had given him so long ago, with personal information he said could be used against Echo.

It was time to read it. If it wasn't already too late."###,
    },
    NovellaChapter {
        id: "s_e11.rtf",
        title: r###"Chapter 11: Reckoning: War"###,
        is_eden: true,
        chapter_num: 11,
        text: r###"Brook watched grimly as Axen locked the helmet of the combat suit in place. The dark, ablative, anti-laser coating showed highlights of iridescent color as he turned, and the light-weight armor plates creaked slightly in response to his movements. Brook

thought of old sims he'd seen, knights armoring up for battle, or matadors putting on their suit of lights.

Brook leaned back against an equipment locker, trying to look relaxed. He felt anything but. "You don't have to do this, you know."

Axen glanced at him, as though just noticing he was there. "The Senate asked me to."

"They didn't order you to, though. Anyone could go. I could go. Frag, half the Senate, Echo's trolls anyway, would just as soon you didn't come back."

Axen held his wrist screen up in front of his face and ran through the checklist. "If I don't come back with the Gene Bank, then nothi

ng I can do here will make any difference anyway. Van Dozier will have won. I have the experience, more knowledge of Plymouth than anyone, and if worse comes to worst, my status as an Elder may offer me some slight extra protection."

Brook wrinkled his upper lip. "I wouldn't count on it. They aren't going to just let you walk out of there with their Gene Bank."

"Has to be done. Besides, if it works, my stock with the Senate, and public opinion, can only rise."

" If you put the right spin on it." He sighed. Axen had always said that, in politics, it wasn't what you did or what happened, it was the spin you put on things. He'd never really believed it until their return from the Masters

base.

Echo's people had started work even before they'd made it back to Eden. By the time they were through, she was the wounded hero of the day, having ferreted out a terrorist stronghold in a courageous undercover mission. Axen's role, and his, were unknown

to the public. Axen preferred it that way, but the situation galled Brook. "Let her have her day," he'd said. "The people who count know what we did, and it might even keep her out of trouble while we deal in more important matters."

Brook didn't think so, though. He'd seen Echo only a few times since returning her to Eden, and then only passing in a corridor. But the look in her eyes on those occasions told him that she hadn't forgotten her threat, and that she intended to follow t

hrough. It was just a question of when.

Through the small view port behind Axen, he could see a Scout roll up and stop outside the airlock. Axen glanced out the window and saw it too. "My ride is here."

"Last chance," said Brook.

He cycled the lock open, stopped halfway through the door, and glanced back with a slight smile. "Mind the store. Whatever happens."

d # # #

There was already someone in the Scout's cabin as he climbed into the right seat and strapped himself in. The cabin pressurized, and as soon as the indicators went green, he flipped up his visor. The other p

erson did as well. He recognized the man immediately. "Dr. Kolo. I knew they were sending someone to assist me, but you were the last person I would have expected."

Kolo smiled. "I'm not surprised. But there are no professional soldiers in Eden, and let me assure you that I lift weights and I'm proficient in several martial arts. I won't be a problem. Also, though Plymouth's lab structures are going to be differe

nt from our own, I think I'm better equipped than anyone to find my way around and help locate the Gene Bank."

Axen nodded. "Agreed. That's not why you're coming though, is it?"

"No. My people are in there, and if there's any chance to rescue them, then I want to help. Besides, the days of serious research in Eden are over. The colony is winding down, coiling for that last leap into space." He turned and looked intently out t

he window, watching the armored convoy that was falling in around them. "If my usefulness as a scientist is over, then I have to find another way to serve."

"Rescuing the scientists isn't a priority. We're going in for the Gene Bank. If they happen to fall into one of our vehicles along the way, so be it, but I won't endanger the mission for them."

Kolo took a deep breath and let it out slowly. "I know. I wouldn't expect you to." He let the thought slip by, smiled, and stuck out his hand. "Forget the doctor business. Like I said, that day is done. My name is Ule."

Axen nodded his approval. This man was going to be watching his back. No sense in formality. "Call me Axen. Good hunting to us both."

d # # #

The RLV Eden Clipper

sat on its launch pad like a vast, manmade mountain, clouds of venting fuel making it look like a barely dormant volcano. The analogy became even more apt as the countdown hit zero, and the big rocket started to belch fire, though from its base rather t

han its top.

Brook paused at the window of the tran-station outside his Residence to watch

the launch. The ground shook as though in one of the ground-quakes that hit them with increasing frequency. Watching the launch vehicle climbing skyward in its tail of fire filled Brook with a mixture of hope and dread. As he studied the faces of peop

le stopped around him, he knew he was not alone in his feelings.

In the enclosed, artificial environment of Eden, human smells were always there. Generally the nervous system simply edited them out, but occasionally, when something unusual happened, it co

uld drive things above the threshold of awareness. Lately it had seemed to Brook that he could smell the anxiety, the fear , of his fellow colonists. In orbit overhead the starship Phoenix Voyager

was nearing completion, and if all went well, soon some of them, some of them , would be leaving.

Who that would be, and how many, was known only to a Savant computer in the CC, which was constantly juggling the crew roster based on a mix of necessary skills, ages, and gender balance. Even the starship's exact c

apacity wasn't known. The stasis units had been designed to allow some doubling up of children and smaller adults, but that would depend on a thousand other variables affecting final payload, and could only be determined at the last minute.

As the RLV disappeared from view, Brook strolled back to his quarters. He'd felt at loose ends since Axen had left with the convoy to Plymouth. His own work had slacked off, the colony was ramping down, not building up, and while he hadn't dismissed the

threat of Ech

o and her friends, her covert activities seemed to have been curtailed by the failed base. He'd just put in several volunteer hours loading the final supply module to be launched into orbit and thought he might finally be tired enough to sleep.

He entered the tiny room, folded the bunk down from the wall, and dropped into it wearily. The rumpled sheets hadn't been changed in a while, he noticed, but he couldn't develop any enthusiasm for the task.

What's the point? The world ends soon enough anyway.

H

e rolled over to face the wall and noticed the message icon flashing on his EnterCom screen. He gave the voice command to play the message. He nearly fell out of the bunk as Echo's face appeared in front of him. "Hello, Brook. Now that the convoy is g

one and observing strict radio silence, I just thought you should know a few things. First, it's all for nothing. Jacque was a decoy, and my other spy in Plymouth will be taking advantage of the confusion created by the attack to destroy the Gene Bank.

"

Second, the reason there will be such a distraction is that he'll have given Plymouth plenty of warning about the attack, so your friend the Elder will walk right into a trap." Her smile seemed to drip venom. "The show's only starting, hon. Don't worry

, though. You'll be here for the final act. Just don't count on an encore." She laughed as the image clicked off.

He rolled out of the bunk and pulled his boots back on. Echo was right in that there was no way to contact the convoy. Even stealthy meth

ods like Axen's tracking module had used were judged to be too risky. The unmanned vehicles were all running on internal programming, and would reestablish contact only when they reached Plymouth or engaged its forces, whichever happened first. This was

a totally black mission, with no possibility of recall.

Brook chewed his lower lip. At this point, it would be easier for Plymouth to contact the convoy than for Eden to recall them. He blinked. It wasn't a good answer, but it was the only one he had. He had to contact Plymouth.

The direct channel would never work. Though there now existed an official communications link with Plymouth, he'd never get access to it in the current political climate. Even if he did, Echo would find out about it immediately and might be able to spee

d up her own timetable. But there could be another way.

For some time he'd had strong hints that Axen had been in contact with someone in Plymouth, and that this contact had a great deal to do with Axen's desire to be reunite

d with Kraft. He'd seen how Axen had rigged a link to Kraft by bootlegging vehicle control signals, and he knew that Axen had made sure Kraft had access to similar control signals for the old observer satellite that the two colonies once shared.

That was it. The satellite was still in orbit, though Eden didn't use it any more.

He immediately put through a priority call to Senator Autzen. "Senator, our mission to Plymouth is in grave danger. The Masters have a spy in the colony. I may be able to save

Axen Moon and the mission, but I need a favor. I can't explain why, but I need Axen's Savant computer linked to the satellite command system."

d # # #

The convoy ran through the twilight and blowing dust as rapidly as it could without lights. Ule Kolo was sleeping, and Axen wished he could. Is that what comes from knowing your work is done?

For Axen, the work was not over. He had unfinished business. There was always unfinished business.

He thought about Nguyen, Echo Van Dozier, his fellow Elders, hi

mself. They all had something in common. They'd tried to do the right thing, and in their arrogance, they'd tried to manipulate factors too complex and dangerous for them to understand. Their sins were all different, and all the same. Nguyen had paid

the ultimate price. Except for Emma and himself, the Elders were all gone now, and as for Van Dozier, her time was coming soon. He was sure of it. For all of them, time was running out.

d # # #

Brook sat on a chair in Axen's quarters, leaning forward, el

bows propped on knees, staring at the inert cube that was Kraft. Once before he'd been able to convince the computer to cooperate with him, but he was delving into new levels of secrecy now, with even less solid data to go on. Also, he wouldn't put it p

ast Axen to install additional instructions to prevent another security breach. Still, this was the only chance he had.

"Kraft, you remember me, and by now, you've become aware of a new data channel that has been opened for me. That's my doing. I know you can use that channel to contact Plymouth."

Nothing.

"Okay, I think

you can use the channel that way. Kraft, you must know about the mission Axen is on, and why neither one of us can contact him. The Masters have a spy in Plymouth and they've tipped him off about the convoy. The Gene Bank will be destroyed, and Axen w

ill be attacked as soon as he approaches Plymouth. I know Axen has a contact in Plymouth. Speaking with that contact is my only hope of saving Axen and the Gene Bank. Help me, please."

Silence. Brook rubbed his forehead. He was at a dead end.

The clockwork icon suddenly flashed on the computer's front surface. "I have located the Masters

secret communications link with their spy. It employed seismic probes to send sonic signals to seismographs in Plymouth. That link has been crippled."

"Kraft! You're talking to me!"

"I had to locate the link before I was convinced of your sincerity. I can establish a link to Plymouth, but I warn you that Axen has not used this channel in some time."

"I'll take my chances, my chance, the only chance I've got really."

"Do you wish visual?"

"Yes."

"Working. I have established a link with a computer in Plymouth. It is now paging the contact."

Brook sighed. Kraft was cooperating, but he could tell that the computer was being less than forthcoming with information. Unless the contact chose to identify him- or herself, he would never know the person s identity.

Several minutes passed before the EnterCom scre

en on Axen's wall flashed to life. He recognized the handsome but weathered woman who appeared on screen from Axen's recent diplomatic assignment. He watched as she casually pushed a strand of salt-and-pepper hair out of her face, but her eyes studied h

im with an intimidating intensity.

"Elder Burke," he said, "I'm honored."

"Well, pup, I'm not. Who the frag are you and where is Axen?"

"My name is Brook Panati. I've been helping Axen with a number of concerns lately." He swallowed. "Look, I don't h

ave much time to convince you of my sincerity. I take it that you're somebody Axen trusts. I hope so, because I'm about to give you some vitally important information you'll want, but in so doing, I'm putting his life in your hands."

Her eyes narrowed in concentration. "I'll be honest with you, pup, I don't think Axen trusts me as much as you think anymore, and I can't make any promises as to what I'll do with any information you give me. Given that, do you still want to talk to me?

"

Brook's mouth was dry. He had no choice at all.

d # # #

The Scout hung back while the armored column rammed head-on into Plymouth's defenses. The crossfire from their defense turrets was brutal, but the whole point was to create an opening that the fast but unarmored Scout could slip through. Lightning flash

ed from the new Thor's Hammer weapon, which Axen had given its first trial by fire. Now that experience was paying off, as first one, then another of the Guard Posts exploded, and a section of the defensive wall bega

n to crumble under the constant bombardment.

Axen took the controls and surged toward the growing opening, bouncing over the rubble just as the wall collapsed. While Axen weaved the Scout between buildings, Kolo rubbernecked, looking for the Advanced Lab. Information clandestinely transmitted to t

hem by the captured scientists instructed them what to look for, and where in its lab the Gene Bank was located, but the scientists' freedom had been restricted enough that they had no idea of the layout of the

colony. Satellite photos helped, but there were still several candidate buildings to visit.

Axen spotted a Plymouth Lynx passing between two buildings ahead, and slammed the tiller over to steer them into a side passage. He didn't think they'd been spotted.

"There it is!" Kolo pointed to their right. They were almost to the building.

Axen switched the Scout over to computer control and flipped the switch to open the hatch. "Grab your gear. Here we go."

They each carried a shoulder bag, a tool belt with holster and pistol, and a light rifle. The Scout slowed slightly as they passed the building, but didn't stop. The two men leaped wide from the vehicle's fender to clear the churning rear tires, and hit

the ground rolling. They scrambled under an overhang at the base of the building and huddled there while they got their bearings.

Kolo pointed back to their right. "The emergency lock is that way."

They moved carefully, watching the windows of neighboring buildings for any sign they'd been spotted. So far, so good. It appeared that most of the citizenry had taken shelter deep within their buildings.

Axen turned his attention to the door's mechanism. The trick was to break in without depressurizing the building. That would sound alarms and be a clear giveaway as to their position and intentions. He placed a small box containing a shaped charge and

radio-detonator over the panel and moved back a meter or so.

He pushed a button on a belt control box and the box popped and fell off the scorched panel, which now had a neatly cut rectangular hole in it.

He clipped a probe over one of the cables and attached it to another small box, which he attached to the wall with a bit of adhesive. He pressed another control on the belt and the outer door of the lock opened.

They both stepped inside, and Axen cycled the lock. The inner doors slid open, and they were in an emergency suit locker. Axen eyed the suits, then started pulling off his combat suit.

Kolo raised an eyebrow. "What are you doing?"

Axen ran his finger along the suit rack until he found one in his size. "Once we get inside, I'll take the point. You hang back and cover me. That way, if I run into someone unexpectedly, I won t appear to be an intruder.

Kolo looked skeptical, but he nodded.

Axen finished dressing in the flimsy Plymouth emergency environment suit. He handed his rifle to Kolo, but kept the belt, side arm, and bag.

They emerged into a narrow service corridor and followed an eq

ually narrow set of stairs up to the main lab level. Even through their closed suits, the sounds of battle surrounded them: alarms, recorded announcements giving directions to shelters, the occasional thump or crackle of a near miss.

With Axen staying three or four meters ahead, they searched the lab rooms one by one. By the information given them, the bank should have been in the first lab, but it was not. With each new door, they came up empty-handed.

Then they checked a storeroom near the center of

the building. The door was locked, but Axen was able to crack it in only a few minutes. The door to the small room slid open, and there was the bank, plugged into a coolant supply panel in the wall and strapped to a two-wheeled dolly.

Kolo slipped past Axen and, after inspecting the Gene Bank, disconnected the cooling lines and prepared it for travel. Axen kept watch through the open door. There was also another locked door on the far side of the room, so he occasionally looked over

his shoulder to check it as well.

"Ready," said Kolo, who stepped behind the bank and tipped the dolly back onto its wheels.

Axen stuck his head out into the corridor, and found himself staring into the muzzle of a pistol. Emma's hand was steady as she pointed the weapon for a killing shot, and she didn't even seem surprised to see him. "Step back inside, Axen, and move away

from the door by your friend, whoever he is."

Axen complied, and Emma moved inside the storeroom and closed the door behind her.

The corner of her mouth twitched slightly, as through trying to smile. "I wish I could say it was good to see you, Axen, but under the circumstances...."

He met her gaze unflinchingly. "I've come for the Gene Bank, Emma. Please don't make this difficult."

This time the smile was genuine. "Oh, this is very difficult, no matter what happens. I heard you were coming, and decided we just had to have a chat."

Axen frowned. "Heard I was coming. How?"

"Your young friend Panati figured out a way to contact me after you left Eden. He sends his regards."

Axen's jaw clinched in anger. Had Panati really sold him out?

Emma shook her head, as though reading his mind. "He couldn't help it, Axen. He found out there was a spy here, representing someone called 'the Masters.' I wish there was time to hear that little story. Anyway, he was going to destroy the Gene Bank a

nd give us an advance warning so we'd be ready for your attack.

Axen tilted his head slightly in puzzlement. If the defenses had been especially prepared for them, he hadn't been able to detect it.

"Oh, I kept my mouth shut, Axen. It wasn't an easy decision, but I knew what you were after and where to intercept you."

Kolo, his hands kept carefully in the air, stepped from behind the Gene Bank. "My name is Dr. Ule Kolo. You have some scientists here who worked for me in Eden. Can you tell me if they are all right?"

She seemed surprised by this revelation, but quickly recovered. "They're fine. They're working for me on a special project."

Kolo was surprised. "Working?..."

"Willingly, I might add. We have some common humanitarian concerns. In any case, they won't be going with you."

"Does that mean," asked Axen, "that you're going to let us leave?"

"Is your ship as far along as your pup Panati says?"

Axen nodded. "And I assume that Plymouth is as far behind as we suspect?"

A look of terrible sadness crossed her face like a shadow. "Yes." She hesitated. "Axen, if I let you tak

e this, it is on the condition that I may approach you for a favor before that ship departs, and that you'll do everything you can to accommodate me."

Axen couldn't imagine what that request would be. He knew Emma too well to think it could be anything as simple or selfish as passage on the starship. "I'll do what I can. You have my word."

She nodded and lowered the gun. "Good." She glanced at the Gene Bank. "It's yours, but I warn you, you won't be able to keep it without a fight. Plymouth was alr

eady planning an all-out attack on Eden to capture your launch facility and, in turn, your starship. They've started converting our launch vehicles into makeshift EMP missiles, that's how desperate they are." Her eyes remained fixed on the bank. "I onl

He smiled sadly. "I will." He looked at her and thought of wasted years. "Emma, I wish it could have been different."

She smiled back, with equal sadness. "I do too, Axen. I " Suddenly her eyes widened and the gun raised to firing position. "Duck!"

Axen went down and to one side, but he still felt the shot as it whizzed past his neck. Before he could reach the floor, three more rounds were squeezed off. As he fell, he turned and saw a muscular blond man crumpling in the now-open rear door.

Then it was all over, and he and Kolo climbed uneasily back to their feet. Behind them, the blond man lay on his side just inside the far door in a growing pool of his own blood.

Emma stared at the gun as though it had just appeared in her hand, then looked up at Kolo. "This is your spy. Your scientist friends were supposed to have taken care of him before he got here. I'd better find out if they're okay." She saw the concern

in his eyes. "I'll get word to you through Axen, let you know that they're okay, but you have to go, now ."

d # # #

Exhausted, Brook unlocked his

quarters and slipped inside. The damaged replica of the Plymouth Gene Bank was planted just outside the colony so that Axen could make the switch before his return. If all went well, the ruse would protect it from further sabotage by the Masters, at lea

st for a while.

He was about to fall in bed when he spotted the flashing message icon. He groaned softly and pressed the icon, anticipating another threat from Echo. Instead, it was an official message from the Savant in charge of launch control.

His request for a spot on the starship crew roster had been denied."###,
    },
    NovellaChapter {
        id: "s_e12.rtf",
        title: r###"Chapter 12: Exodus"###,
        is_eden: true,
        chapter_num: 12,
        text: r###"Brook was incredulous. "Human factors? What the frag does that mean?" In frustration, he kicked the recycle bin across Senator Autzen's tiny office, and immediately felt like a childish fool for doing so. Yet if he was going to die, he had to know why. Even Axen, as many enemies as he had made, had been given a boarding pass. Why me?

Senator Autzen was sympathetic, but unyielding. "Son, you want the bureaucratic line, or the straight truth?" He read the answer in Brook's face. "The truth is, somebody with power doesn't like you. The Savant had you on the preliminary crew roster, but there's a 'tweak' built into the program to allow some override of the system. In theory, there are some human factors that the computer might not be able to understand or appreciate and that allowed for adjustment. In practice, it's been used in a number of cases, including yours.

Echo it's the only answer. This is the revenge she promised.

"You've been a great help to us recently, Brook. You've fought the good fight and done well. I wish I could reward that." Autzen shrugged his shoulders. "But there isn't anything I can do about it privately, and if we go public the entire crew selection process could devolve into chaos. We could all be at one another's throats before we knew what was happening. If it's any consolation, I'm not going either; none of us on the Senate are. We declared it a conflict of interest and removed ourselves from the running."

Brook looked at the man, the tiredness in his eyes, the lines that had appeared in his face in the last few months, the slight tremor in his once strong and steady hands, and found himself regretting that he'd come at all. "I'm sorry, Senator, I didn't know. You've got kids?"

He nodded. "Two. It's hard for them, for us all."

Brook stood and self-consciously repositioned the recycle bin. "Just forget I was here, Senator. I'm sorry to have bothered you." He paused at the door. "You know you can still count on me, for whatever it takes."

He smiled sadly. "I know we can, son. And thank you."

d # # #

Nobody knew when the Gene Bank would be transferred, including Axen. On the assumption that secrecy was the best way to safeguard it from the Masters, they'd created the false Gene Bank and the cover story that it had been damaged beyond repair during the return from Plymouth. Axen hadn't expected that story to hold them off forever, but it gave him time to hide the real item and arrange for its safe removal to the starship.

Axen knew that the fewer people who knew a secret, the safer the secret was, so he'd confided only in a trusted few Brook, Kolo, and Autzen and had recruited these three to assist him in the move. But since none of them would have advance warning of the event, there was no possibility of the information leaking out somehow.

He'd instructed Kraft to generate a randomly-timed message before a randomly-chosen launch, and that message had popped up hours before. Then the first flaw in the plan became apparent. He wasn't able to contact Kolo. The scientist wasn't in his quarters, or in the largely mothballed Advanced Lab, where the Gene Bank had been hidden in a case marked "Geological Instruments."

Brook had seemed only mildly concerned. "He's probably found himself a lady friend that he's staying the night with, Axen. Not everybody can be as saintly as you are, or as unlucky as I am."

But Axen's internal alarms all went off at once. Every step and turn of the trip from the lab to the Space Center he'd been braced for trouble. His hand never left his pocket, where it gripped a fully loaded pistol, his senses scanning for anything the slightest bit out of the ordinary, but they reached the RLV loading platform without incident.

Four armed Volunteer Guards flanked the yawning cargo hatch in the big rocket's side. He scanned their faces, and took some comfort in the fact that they were all familiar, all people he'd worked with before. A technician checked off their cargo manifest and authorization on his ClipCom and directed them to a location where the crate could be attached to the deck with quick-release bolts, then disappeared back to his station by the hatch.

Axen watched closely as Brook fastened and triple-checked the bolts. He shook his head. "This just isn't right. I have to see it with my own eyes."

Brook looked confused, but stepped back out of the way as Axen unlocked the crate lid and removed a false lid designed to simulate the top of the equipment listed on the manifest. Under it, the actual top of the Gene Bank was exposed.

Another key and lock were required to open it. Vapor from the cold, liquefied gasses inside boiled over the side of the box and cascaded to the deck like a frosty waterfall. He produced an insulated glove from his pocket and put it on. Using the protected hand, he unlatched the mechanism and slid it up out of its hyper-cold bath.

The Gene Bank's complex innards were revealed, tens of thousands of tiny glass vials housed in a complex carousal mechanism capable of delivering any given vial by number. A large insulated can at the top housed a motor and drive train that powered the whole thing.

"It looks fine to me," said Brook.

Axen squinted and looked closely. It did look fine, and it was certainly genuine. Unlike the original Earth-made bank that had been destroyed, this one had been handmade in Eden just before the dissidents left to found Plymouth, and no complete plans had ever existed. The replica they'd created had been designed only to fool eyes that had never seen the original. Axen had studied this one carefully before committing it to its hiding place.

"It's genuine." He slid the mechanism back into the case and sealed the lid before any thermal damage was done. His fingers played over the small control panel that projected from the upper side of the box. "All the more reason I think the Masters are up to something."

Brook's brow wrinkled in concern. "You have any idea how to find out what?"

Axen nodded. "We'll ask," he said.

d # # #

The familiar Structure Factory control gallery was dark and unnaturally quiet. No new structures had been produced in months, and the plant might never be used again. That made it perfect neutral territory for their meeting.

Axen and Brook stood on one side of the platform, Echo and Gi on the other. Everybody had a gun. Nobody was talking.

Echo broke the silence. "Waiting for something, Elder?"

Axen said nothing, but just then there was a rumble, and the building shook slightly. On hearing the sound, Echo smiled for no apparent reason.

"That would be the Clipper taking off. It's safe to talk now," Brook said.

Echo was still smiling. "Safe to talk about the Gene Bank? Yes, I suppose it is."

Brook's mouth fell open. Axen just met her eyes unflinchingly. "You knew?"

She laughed. "Your fake was very convincing, even after we'd inspected it closely. There was even human genetic material in the remains of the vials." Her face went serious. "It was too good. I analyzed the genetic material. All of it had been reproduced from a single sample, and as you can imagine, I had no trouble determining whose."

"Dr. Kolo," said Axen.

"The fake had to have been crafted in one of the labs, we knew that, but the sample told us who had done the work, and from there, the rest was easy."

Brook stepped forward, anger on his face. "Where's Kolo?"

"He had an unfortunate accident, several of them actually, until he told us what we needed to know. By then though, the cumulative effects were fatal."

Axen felt his stomach knot. One more death on his conscience. He glanced warily at Brook. The young man looked mad enough to do something stupid, and there'd been enough innocent deaths today.

"So you found the Gene Bank and sabotaged it. How? It looked fine when I checked it before launch."

She laughed, deep and hard, until tears ran down her cheeks. Only Gi's vigilance kept them from subduing her. Finally she managed to calm herself a bit.

"What's so funny?"

"The irony. Your precious Gene Bank is fine. I took it for granted that you wouldn't be any less thorough in your inspection than we were. If the Gene Bank had been less than perfect, we might not have been able to smuggle the bomb that we installed in its motor housing onto the RLV, and in turn onto the starship."

Brook's eyes were wide. "You used the Gene Bank as a Trojan horse, so you could use your bomb to hijack the ship."

She laughed again. "And now the starship is ours. We won't let you leave here until the box is safely stowed on the starship."

"Put down your guns, slowly," said a woman's voice from behind them. Axen glanced over his shoulder to see a muscular woman aiming a rifle at them from the rear stairway. He recognized her as one of the three Masters Brook had identified from his first meeting, the one named Sharon.

"Do what she says," he instructed Brook, as he laid his own gun gently on the deck and pushed it away with his toe. He looked back at Echo. "You're going to kill us now, I suppose?"

She smiled, clearly enjoying the moment. "I might kill you, Elder," her attention turned back to Panati and anger flashed in her eyes, "but not you, Brook. I want you alive to watch the Clipper 's final departure with me on it. I want you left here to watch the light in the sky as the starship fires its engines and pulls out of orbit. I want you here to see the flash when I kick your precious Gene Bank out the airlock and blow it to atoms. I want you here when the Blight rolls over the horizon and melts you into a bag of pus."

"When the Blight gets me," he answered, a new calm in his voice, "you'll be right here to enjoy the show. Your bomb isn't on its way to the starship. It's about half a meter under the platform where you're standing."

Reflexively she looked down, then caught herself. "You couldn't have removed it so quickly. There were too many booby-traps."

"We didn't," said Axen. "When I realized the mechanism didn't work, I guessed you might have done something of the sort, and decided to let you tell me what your plan was. Loyal members of the Volunteer Guard have been monitoring this whole conversation remotely and are watching all exits from this building.

Echo dug frantically in her pocket and produced a small card device. "I can still destroy the Gene Bank, with you two, and it, as hostages. We can go wherever we want, do whatever we want. You still lose."

"Only if you're all willing to die for the cause of glorious genetic purity." His eyes locked with Gi. "Are you Gi? Do you believe that strongly?"

"Yes," he said with only a slight hesitation.

"You, Sharon?"

"Completely," she replied. There was no hesitation at all, either in timing or tone.

That settled it. His only hope was to work on Gi. He made eye contact with the big man again. "I suppose you think Van Dozier believes it as well, don't you Gi?"

He frowned, suspicious, but nodded.

Echo realized they were up to something and waved her gun at him. "Shut up, both of you. Gi, Sharon, don't listen to them."

"Is there something you wouldn't like them to hear, Dr. Van Dozier? Some personal secret?"

" Shut up! "

"You're all clones in the inner circle, aren't you Gi, the 99.9 th percentile people, the super-race?"

Gi said nothing, but he was listening intently.

Axen continued, "I suppose Echo told you she was one, too, maybe even showed you documents to prove it, but those would be easy enough for her to fake in her position."

The look on Echo's face showed that she'd finally realized what he was about to say. Axen half expected her to shoot him then, which might have served his ultimate purpose as well as what happened. "It's a lie. Tell them, Brook."

Brook looked at her, almost apologetically. "Her parents were Elders. She was conceived naturally. Her father died before she was born, and she and her mother nearly died during the famine three years after landing." He hesitated. "Her identical twin did die."

Tears were streaming down Echo's face. Her gun hand trembled. "No."

"She's just one of the genetic mongrels your movement professes to hate, a hurt, broken one who has spent her whole life trying to replace a twin she can barely remember."

Echo shook her head. "No, no." Then she fired.

Things started to happen all at once. Axen ducked as the muzzle of Gi's gun swung up toward him and fired. Axen hit the deck and rolled, unsure whether either bullet had hit him.

Brook jumped toward Van Dozier, and her gun fired again at point-blank range. The bomb trigger flew out of her hand and landed on the deck near Axen, who grabbed it and shielded it with his body.

Axen was looking for the gun when he realized that Gi was lowering his own weapon, and was paying no attention to him. Gi's attention was fixed on the rear stair, where the woman known as Sharon hung limply over a railing, blood running down her arm and over her face in a thick cascade.

Van Dozier pushed Brook away from her. He flattened out on his back, revealing the blackened hole in the front of his shirt, and lay unmoving.

Van Dozier raised her pistol to shoot Axen.

Gi spun and shot her cleanly through the forehead. He watched her fall, then turned and handed the gun, butt first, to Axen.

"I think I've made a mistake," he said.

d # # #

Axen slumped into the lone chair in his quarters and looked around wearily. He couldn't remember the last time his life had been this simple. Most of his work was done the debts that could be repaid, had been, all but two.

He looked at Kraft. This was the first one. "Kraft, open a channel to Emma, full visual."

It took almost twenty minutes before her face appeared on the screen. He didn't move in all that time. "Emma, if you have a favor to ask of me, there has never been a better time."

She smiled that sad smile. "Good, because I'm finally ready to ask, and to offer a little gift in exchange, something Frost, your missing scientists, and I were able to cook up over the last few months. Kraft, please display the data we're sending for Axen." Another window opened on the screen, displaying a series of schematics and formulas.

"What is it?"

"A minor modification of your starship's ion engines to improve the efficiency. It should increase your payload capacity by ten to fifteen percent with no additional fuel."

Axen was surprised in spite of himself. "Why are you giving us this, Emma? Why squander your resources on something you had no way to apply?"

She chuckled. "Because I have a use for that extra payload, Axen. I'm sending you some passengers."

d # # #

Brook lay in the med-station bed watching his own life-signs flickering on the screens. He would have preferred to do most anything else, but it hurt to move.

The door slid open and Axen walked in. He leaned over the bed. "How are you doing?"

"For someone with three broken ribs and possible internal bleeding, not bad."

"It could have been worse."

Would have been worse, if he hadn't stuffed those armor pads, taken from a combat suit, under his shirt as Axen had insisted. He felt rotten, and he found himself smiling. "We did it, didn't we?"

Axen smiled back. "We did it. Even if Plymouth's forces overrun us, the Gene Bank is safe on the starship. We've saved everything that humanity is, given it another chance." He patted the young man gently on his arm.

Then his expression changed, as though he remembered something almost forgotten. He reached into his pocket. "I brought you something." He removed a black plastic square and placed it in Brook's hand.

Brook rolled it over and looked at the Eden logo embossed on the surface. It was a boarding pass. He looked up at Axen. "Where did you get this?"

"It's mine."

"What?"

"Senator Autzen was able to shuffle some records without drawing undue attention. It has your name on it now."

"I can't take this." He tried to hand the pass back to Axen.

Axen just waved it away. "I never intended to go. I'm old, at least compared to the rest of you. I managed to duck hibernation sickness the first time, I doubt I'd be so lucky the second, even with the improved methods. It's your turn."

He stood and started to leave the room. "Just remember, I've been where you're about to go. Don't be so sure I'm doing you any favors."

\"###,
    },
    NovellaChapter {
        id: "s_e13.rtf",
        title: r###"Epilogue: New Horizons"###,
        is_eden: true,
        chapter_num: 13,
        text: r###"It had been a good day, thought Axen Moon as he stood on a hilltop, watching the Eden Clipper climbing into the sky for the final time. So much accomplished, so much to remember.

There was the moment when he tried to imagine what the people guiding Plymouth's forces thought, when one of their own Evac Transports charged through their lines and headed into Eden under the protection of Eden forces. What did they think when they learned that this transport carried all the remaining children of Plymouth?

Those children were Emma's "passengers," her bounty for the improved ion drive. It was a boon gladly given when Axen had taken the offer to the Senate. Not one of them even questioned how the offer came to be made or Axen's role in the matter.

He thought of his final words to Brook Panati as they stood on the boarding platform for the Clipper . "This New Terra is my world, the world I and my kind built, and this is where I should stay. But this new world, no matter where it is, no matter what it's called, is your world. Learn from this. Be better than we were."

They shook hands for the last time, and there was a moment between them when no words were spoken. Axen had imagined a different world, where he and Emma had raised a son like this, and he had been for that moment, proud.

He stood on the hill overlooking the abandoned hulk of Eden. On the far edge of the colony, buildings exploded and burst into flame as the Blight advanced in its invisible, inexorable way, life and death advancing as one.

He looked up at the sky, tinged with blue, decorated with thin ribbons of icy cloud. This was the time. All afternoon he'd been adjusting the atmospheric settings of his suit, decreasing the oxygen content, slowly lowering the pressure. This would either kill him, or not.

He unfastened his helmet with a hiss. His ears popped and he held his breath as he removed it. The air was cold as though he'd put his face in the Gene Bank, but he ignored it. He took a breath of Eden's air.

It was thin, cold, unsatisfying. It burned. But it was his air. His world.

He was dizzy, though he wasn't sure how much of that was physiological and how much emotional. He replaced the helmet while he still had the strength to do so and snapped it back in place.

His ears popped again as the suit refilled. He'd be lucky if he didn't get the bends. It didn't matter.

He took one last look at the dying Eden, and turned away, toward the convoy of survivors who waited below, from Plymouth and Eden both, their differences forgotten.

Emma was down there.

He walked to join them. Perhaps to lead them. The Blight might be as unstoppable as they thought, but while there was life, there was hope. Now that the survival of humanity was ensured, it was all they had.

They would never give up.

They were only human.

\"###,
    },
    NovellaChapter {
        id: "s_p01.rtf",
        title: r###"Chapter 1: Meltdown"###,
        is_eden: false,
        chapter_num: 1,
        text: r###"Emma Burke climbed down the ladder from the Robo-Surveyor and felt the muffled crunch of New Terra's porous soil under her rocksuit's boots. After three days, the air inside her helmet was stale, and tasted metallic as she took a breath through her mouth

. The vehicle's small passenger cabin was intended for out-and-back day trips, not extended expeditions.

"Did you feel that?" Wu Chen's voice crackled in her helmet speakers. She turned to see the pale green head and shoulders of Wu's rocksuit poking out of the Surveyor's hatch.

"Feel what?"

"Another tremor. Just picked it up on the seismograph. Might have been too weak to feel."

She frowned. Weak or not, it shouldn't have happened at all. New Terra's crustal plates were nearly locked, its faults all old, at least in human terms, and inactive. That was part of the reason for their trip, to plant a ser

ies of automated seismic stations in order to study the quakes.

The other reason, unknown to anyone but Emma, was to search for evidence of Eden's clandestine terraforming experiments. She thought back to her last message from Eden a few weeks before. Though the colonies had officially broken off communications and

the comm satellite was disabled, she and fellow Elder Axen Moon had maintained a secret link through a weather-satellite telemetry channel. His last communication had warned that Eden's leade

r, Nguyen, had begun weapons development, and had unilaterally started a program of biological terraforming.

Then, something had gone wrong, and the link had been cut off. Axen and Nguyen had clashed on numerous occasions. Was he now a political prisoner, or worse, had he been killed? There was no way to know, and she could only be certain that things had take

n a dark turn at the rival colony.

Unfortunately, that put Emma in a difficult situation. Maintaining her secret link with Axen could be consider

ed a treasonous act. At best, the colony's leaders would never trust her again, and she couldn't have that. She had to find a way to alert them to the danger at Eden without showing her own hand. If she could create concern, any kind of concern, there

was still an observer satellite in orbit that could be used to spy on Eden, and the rest would come out soon enough.

A flash of motion at the edge of her vision caused her to glance up at the ruddy slope of the hillside that loomed over the Surveyor. A ro

ck the size of her fist bounced down the slope and ricocheted off the roof of the vehicle. It was nearly soundless in the thin air, only a soft crack as it hit the Surveyor reaching her external microphone. A cascade of sand and loose pebbles slid down

the hillside, and suddenly the ground heaved under her feet. Her eyes widened.

"Wu! Get the Surveyor out of here! Get away from the hillsides!"

She saw the vehicle surge backward a few meters, the hatch still hanging open, then hesitate. "What about you?"

"Move! I'll take care of myself! We lose the Surveyor and it

s a fifty kilometer walk!" Even as she said it, she was running away from the slope, looking for some kind of shelter. They'd come half a kilometer up a box canyon between two hills following s

ome strange magnetometer readings. Now that canyon was a trap they had to escape. She glanced up, and just as she'd expected, a shower of boulders, some as big as the Surveyor, was bouncing down the hillside like a dinosaur stampede.

She caught a gli

mpse of the Surveyor as it raced out of sight, then began a serpentine run, dodging the small rocks that were even now tumbling past. She jumped just in time to avoid having a half-meter rock cut her feet from under her, then angled toward what appeared

to be a cleft in the ground rock ahead. Great, she thought, there's a quake and I'm thinking about climbing into a crack in the ground.

There was simply no choice though. She glanced back in time to see a boulder as big as a Residence rolling end-over-en

d directly toward her. The cleft wasn't much more than a meter deep, and narrow enough that her rocksuit didn't want to fit, even as she tried to slide in sideways.

She grunted trying to squirm into the hole, even as she saw the huge rock bearing down on her. For once, she wished she'd worn a regular pressure suit rather than the bulkier, reinforced rocksuit.

The boulder was only a dozen meters away now, the upper end of it looming over her. Then something snapped, and she slid easily down into the cleft just as the huge rock dropped on her like a hammer.

She never would have thought that solid stone could ring, but it did, like a bell, the sound being transmitted by contact between her helmet and the rock, so loud that she thought it might deafen her. She closed her eyes against the pain, and opened them

to darkness. Great Maker, I'm buried alive!

Then, a low rumble, and the huge rock rolled over one last time, seeking an equilibrium. Though its flank reared right over her head, there was plenty of room for her to escape.

She remained motionless for a time, until she was certain the quake was over, then tried to climb out of the cleft. For a panicked moment she thought she was stuck, then, with a lurch, the upper part of her suit came free, and she was able to climb up on

to the lip.

She sat there, feet dangling into the hole, waiting for her racing heart to slow. It was only then that she noticed how clean the break was. She ran her gloved fingers over the sharp edge of the break. It lacked the

characteristic billion-year-old orange oxidation that covered the rest of the rock surface. Then she bent to examine the black residue in a vertical line in the cleft wall. This crack wasn't natural. It had been blasted open.

An unintelligible voice crackled in her helmet. Wu was coming back for her. "Wu, I'm okay." It was unlikely that he'd be able to understand her, with her weaker suit transmitter, but there was at least a chance he'd hear enough to know she was still ali

ve.

She followed the clef

t in the rock a few meters. It narrowed, then abruptly widened. Now that she knew what to look for, she could see the marks made in the surrounding rock by vehicles and machinery, those that hadn't been scoured away by New Terra's thin, but furiously fa

st, windstorms. It was in the wide part of the crack where she found the drill cap hidden.

The metal cylinder, painted reddish brown to match the rock, was a stubby metal plug as tall as her waist, though the blunt top of it was almost level with the surface of the rock. Whoever had put it here clearly hadn't intended for it to be found.

The design was familiar to her, a standard cap for a drill hole created by one of the Robo-Moles used in mining operations. Notches automatically engraved in the top cap by the drilling machine provided information about the depth of the hole, type of dr

illing, and other data. A glance at it told her that this was a very deep shaft, several kilometers, and nearly straight down, not at all the kind of hole one would use for mining.

She sat down next to the cap to wait for Wu's return. She ran her gloved hand over the cap and the ring of slots around the top designed to vent any underground gasses that might be generated. As she did, she saw dust dancing along the surface of the ne

arby rock face. Not believing her eyes, she grabbed a fistful of dust from the bottom of the cleft and held it near the slots. Slowly, layer by layer, it was blown away by the outflow of gas. She didn't have a

way to test it, but she was certain that testing would reveal oxygen and water vapor, somehow being generated deep in the rocks, possibly by bacterial action.

This was the evidence she'd been looking for, this was hard proof of Eden's actions, evidence she could present without creating suspicion.

"Emma, you're all right?" She glanced up to see the Surveyor roll around the end of the huge rock.

"I'm fine, Wu. Get the cameras and the sampling gear. We have work to do before we head home."

d # # #

They'd

driven all night, as fast as the terrain and their headlights would allow. Emma took the first shift monitoring the autopilot, helping it to make difficult decisions about which path to take, then dozed off in the wee hours of the morning as Wu took ove

r.

They'd been unable to raise Plymouth by radio. She'd tried to reassure Wu that the relay on top of Mt. Goddard might have been taken out by the quake, but judging by the constant tremors their seismic stations were reporting, she wasn't so sure.

"Boss, wake up."

She moaned and blinked the sleep out of her eyes. She sighed. "Where are we?"

"About twenty clicks out. Once we come out of the canyon we might be able to see something."

"From twenty clicks? I don't think so." She glanced at Wu, and didn't like the look of the frown on his boyish face. "Wu, what's wrong?"

He sighed. "Just before we entered the canyon, I saw an orange glow on the horizon. I thought it was sunrise, until I checked the chrono and realized it was twenty minutes early."

She shuddered involuntarily, but said nothing, her attention focused on the forward view port. The sun was up now, the rocks redder than usual in the harsh morning light. The Surveyor topped a rise at the lip of the canyon and whirred to a stop.

Wu leaned forward, his eyes wide, his mouth open. "Maker, they've blown it up. Those Eden fraggers have blown it up!"

Emma stared at the huge brownish fountain that towered into the sky in front of them, more a plume than a cloud in the thin air. It looked di

fferent from the pictures she'd seen in the geology database, but those pictures had been taken in Earth's thick atmosphere. "It's not a bomb, Wu, it's a volcano."

"A what? "

"I don't know how, or why, but pray there's a Plymouth to go back to. Mt. Goddard has erupted.""###,
    },
    NovellaChapter {
        id: "s_p02.rtf",
        title: r###"Chapter 2: Recovery"###,
        is_eden: false,
        chapter_num: 2,
        text: r###"The Robo-Surveyor sat on a bluff overlooking the remains of Plymouth. Below them the scene was hellish, the few structures that remained intact floating in a lake of lava, like raisins in a bowl of oatmeal. Even as Emma watched from her vantage point standing on the vehicle's fender, the last Residence exploded in a cloud of fire and self-generated steam. The clouds slowly drifted away, leaving the cored-out hull of the building to slowly slump into the lava.

Much as it hurt, she couldn't stop watching. It somehow seemed appropriate that she might be the last person to gaze upon the dying colony, the rest of the survivors, except for Wu of course, long having moved on to a rendezvous point thirty kilometers east.

The Surveyor's hatch swung open and Wu climbed out, his rocksuit blackened and scorched in several places, as was hers, a result of their near-disastrous return to the doomed Plymouth. He didn't look down at the colony. She'd noticed that he intentionally averted his eyes every time the wreckage was in view. "I just talked to the chairman. They need us back with the group. We have to help locate a new colony site ASAP."

She sighed, but said nothing.

"You can't sit out here forever feigning a problem with your suit radio. The Council wanted to talk to you, and they're getting insistent." He leaned back on the vehicle's superstructure, carefully turned away from the sight of the colony.

Through his visor Emma could see that he looked pale, his normally cheerful face looking tired and numb. His brown eyes studied her intently, and she didn't enjoy the attention. He held out the right arm of his suit, where the overlayer of duramesh had melted into the outer pressure layer.

"You owe me an explanation, Elder. We nearly got fried getting out of the Command Center, and you still haven't told me why we had to go back there in the first place. What's on that data-slip that you'd risk your life, both our lives, to get it?"

"Satellite pictures of Eden. While we were in the Command Center I was able to redirect the observer satellite to train its sensors on Eden. The orbital position was less than ideal, but they may tell us something about what's happening to New Terra, and what they're up to with these blasted terraforming experiments."

"You think they have something to do with the sudden geological instability?"

"I don't see how, but this could be an attack of some kind."

"Attack? Why would Eden attack us?"

She shook her head sadly. "You are young, aren't you?"

"But Eden doesn't have weapons, much less something that could cause a volcano to erupt."

Emma suddenly realized that she'd overplayed her hand. It was an uncharacteristic slip for someone so used to deception, but she was under a lot of stress. Or was it more than that?

Her thoughts turned inward, coldly reviewing her own motivations like a shopkeeper taking inventory. Did part of her want to open up, to share her load of secrets with someone? If so, Wu was a poor choice.

Not that she didn't trust Wu. She'd developed considerable respect for him in the time he'd acted as her assistant, but he was too young and immature for the burden she carried. Perhaps someday, but not today. Not today of all days. With all communication to Axen cut off, there was only one left she could share her secrets with, the one who had shared them from the beginning.

She stood and brushed the red dust off her rocksuit. "Take a walk, Wu."

He just stared at her. "What?"

"Take a walk. I need some time to talk with Frost. Alone."

His frown deepened into a scowl, and his brow knotted. "That's it! Elder, with all due respect, you owe me an explanation. You know things you aren't telling me. Out in the wastes, when we found that well, you knew what you were looking for. You know something about Eden, too." He paused to nervously wipe a film of dust off his visor. "You may have your reasons for keeping things from the Council, but," he waved his arm in the direction of the remains of Plymouth, "I've followed you into the pit of hell based on nothing more than your word that it was necessary. I trust you. Why can't you trust me?"

"I do trust you, but if you really trust me, you'll take a walk."

He seemed to waiver, but didn't move.

"Some things are beyond trust, Wu. You'll understand that someday."

Wu slowly turned and climbed down from the Surveyor. He trudged away, his back toward Plymouth.

As she opened the hatch to the Surveyor, she called to him. "Wu, I've put a lot more confidence in you than you know. Try to keep that in mind."

He said nothing, but she thought she saw him nod through the glare on his faceplate. Then he turned and continued his listless walk.

Emma climbed inside the cramped compartment, closed the hatch and dogged it shut, then touched the "pressurize" control. A high whistle, slowly growing in volume, indicated the return of atmosphere to the compartment. She swiveled one of the two seats around to face the rear wall, and sat down. The whistle softened to a low hiss, and a chime and green indicator lights indicated that the air was now breathable. She touched the stud on her helmet that allowed her to open it like a clamshell and removed it.

The air was cool and had the faintly canned smell of an airlock. Her rocksuit itched, and she wished she could take it off, but that wasn't practical in their limited quarters. She unhooked and removed her gloves and reached over to put one hand on Frost's smooth, black upper surface.

The cubic Savant computer was tilted back at an odd angle, its rear corner tucked into an equipment bay to save space. A crude framework of welded angle-iron supported the computer and secured it in place.

The mechanics had at first balked when she had asked that the Savant be installed in the vehicle, but she'd insisted they needed it. In truth, she simply didn't want to be that far from the computer, and the secrets that it guarded.

Now that instinct had been vindicated. If she'd left Frost in the colony, it would have been destroyed along with her Residence before she returned.

She wondered if the computer could feel her touch, and supposed that it could. Its entire outer surface was both sensor and display.

"Frost," she said softly.

The computer remained black and inert.

"Frost," she repeated, a little louder.

A rectangular window came to life on the front surface of the cube. In that window was a translucent snowflake, Frost's identity icon. "I'm sorry Emma, I was dreaming. There is so much to dream about right now."

"Dreaming" was what the Savants called their deep-thought mode, when they shut down all but the most basic inputs and outputs and focused their protein-based computer elements on difficult problems.

The analogy wasn't entirely inappropriate. The process was ill understood, even by the Savants themselves, and the results, especially on large, chaotic problems, were often vague and difficult to interpret. Despite that, the results were often useful. Frost was aware of the disaster at Plymouth, and given the limited information available, had been considering the situation.

"I know," said Emma, "and I've avoided disturbing you until now."

The snowflake icon changed slightly. From years of experience, Emma could read these changes like facial expressions, and this particular change indicated surprise. "While we were talking I reviewed the Surveyor's sensor records for the past eight hours. It seems I missed a great deal. The data collected while we were inside the colony may be quite useful."

She smiled slightly. "When I made the decision to take us back into Plymouth, you were already in deep-thought, and I didn't want to frighten you unnecessarily. There was little you could have done except observe."

The snowflake changed again. This time, the emotion was obscure to her. "In my case, 'frighten' is an inaccurate term. To 'cause concern' would be more exact. You do tend to over-anthropomorphize."

"Perhaps." She didn't want to get sidetracked onto the sort of philosophical discussions that she and Frost often engaged in. "So, did your dreaming produce any results?"

The snowflake changed to reflect annoyance and frustration. "The data was quite limited. My only observation is that there is an ongoing danger here, and that it is unlikely to be localized. I recommend caution in choosing a relocation site for Plymouth, and that it be placed a considerable distance from here, and as far from the terraforming well you discovered as possible."

"Assuming the well represents some kind of threat, there could be other wells that we aren't aware of."

"Probably, but the well you discovered was in the direction of Eden, and it is likely that any other wells are also in that direction. In any case, that is a logical rationalization for the results of my dreaming. The results of a dreaming are not created through observable logical processes. I can only tell you what my dream tells me, not why, especially with such a difficult problem."

She unzipped the pocket on her suit's left leg and reached inside. "Perhaps this will help."

"Ah, the data-slip you mentioned."

She stopped with the slip halfway out of the pocket. "How did you know that?"

"All suit radio communications are stored in the vehicle's short-term memory. I reviewed them along with all the other data."

Even after all these years, Frost was still full of surprises. Perhaps the computer had learned from its owner a little too well. "I didn't know that. In the future, make sure that all such memories are purged if any opportunity exists for anyone else to access them. Especially make sure they are purged before we return to the colony."

"Certainly, Emma. I have already done so. The contents are already stored in my internal memories anyway. Now, may I see that data-slip? Have you inspected it yourself?"

"I barely had time to redirect the satellite, make the recording, and escape with my hide intact. Display any interesting data for me, please. I'd really like to know what that lunatic Nguyen is up to." She placed the slip on Frost's upper surface, where it clung as though by static electricity. After a moment there was a crackle indicating that the slip had been erased. She removed the slip and tossed it in the vehicle's recycler.

The snowflake changed to an even more extreme expression of surprise. "Our last transmission from Axen indicated that there had been a lab explosion; however, there was no reason to believe that it wasn't localized."

She glanced out the vehicle's view port to see Wu standing a few dozen meters away. He made a pointing gesture at the vehicle, wondering if he could return. Emma ignored him. "Frost, what do you mean?"

"There has been extensive destruction affecting approximately eighty percent of Eden's structures."

She couldn't believe what she was hearing. It didn't make sense. "Show me."

The snowflake was replaced by a larger window displaying the satellite pictures. The view covered most of Eden's main colony. It was much larger than she remembered, having expanded and rebuilt considerably since the Plymouthers took their toys and moved elsewhere. The picture had the gray, flat look typical of satellite photos, and there was considerable distortion, as Eden had been near the horizon as seen from the satellite, all of which made it difficult to interpret the image.

She could make out, roughly anyway, the familiar shapes of standard Eden structures, though there were some new ones that she didn't recognize at all. But the roof coloring and roof-lines were wrong. Some were darker than they should have been. Others had a streaked, puckered appearance. Small, bright flecks dusted the spaces between buildings.

The view zoomed closer, narrowing down to a dozen or so buildings. Or, at least, what was left of those buildings. They were ripped, scorched, sometimes literally blasted open within, the roof peeled back like flower petals, pieces of hull metal and building contents scattered everywhere, the light flecks she'd seen before.

"Great Maker. Frost, is this as bad as it looks? How many survivors?"

There was an uncharacteristic pause in Frost's response. "Frost?"

Frost made a strange noise, rather like clearing the throat. "I find no signs of life remaining in the colony. I see no signs that the wreckage has been salvaged. I find no evidence of survivors."

Emma leaned back in her chair, feeling a cold knot in her stomach. She thought of the tenuous thread on which Plymouth's survival hung, and now this. She remembered, though she'd only been a child at the time, the stirring speech that the captain of the Conestoga had made to the colonists before the starship left Earth orbit. He'd closed it with the words "Extinction is not an option." It had been an unofficial motto for the earliest colonists, and until today, one could still find it hanging on the walls of many of Plymouth's living quarters.

Perhaps not, but at this moment, it seemed a distinct possibility.

\"###,
    },
    NovellaChapter {
        id: "s_p03.rtf",
        title: r###"Chapter 3: Revelations"###,
        is_eden: false,
        chapter_num: 3,
        text: r###"The grainy image covered every wall of the darkened Council Chamber, randomly blasted buildings, overturned vehicles, scattered wreckage, and things moving in the blind, shambling way of the dead.

"It gives me the chills looking at it," said a man's voice.

Emma recognized it as Councilor Kozu. "I've watched it a thousand times these last few months," said Emma, "and I'm afraid it affects me the same way. The disaster at Plymouth we can at least understand the nature of, if not the reason why, but this is

unimaginable. The colony is simply deserted, the destruction caused by, as best we can figure, random folly, and these vehicles wandering around."

"You're sure they're armed?" This time the voice was the husky, authoritative voice of Councilor Salish. Emma could just make her out in the darkness, her eyes glittering in the reflected light of the screen.

"We have seen them fire what appears to be a laser on at least one occasion. We believe this was triggered by a bit of windblown debris from one of the damaged buildings. In other words, they are motion sensitive, and not at all discriminating. It's ha

rd to understand how such a stupid robot could be considered an effective weapon, defensive or offensive, but there's obviously something about this that we don't understand."

Another woman spoke, not one Emma recognized, probably one of the new Councilors elected to replace the two killed in the disaster. "Have there been any other attempts to send a vehicle into the area?"

"No, our first few attempts were destroyed before they could approach the Eden site, and before they could return much useful data. We do know that the second vehicle, which was sent in weeks after the first, didn't get as far before it stopped transmitt

ing. That suggests that whate

ver is causing this damage is spreading, and gives us an approximate rate of spread. It seems pointless to try again unless we know more about what this menace is and how it works. The fact that some of the machinery in Eden continues to function, most

notably the power plant, suggests that there may be some ways to resist the effect."

Kozu asked, "Could this be related to the increase in geological activity, or Eden's reported terraforming experiments?"

Emma sighed. "It's very premature to make any su

ch pronouncements. We know they were working on terraforming, and that this could somehow be related to our volcanic problems, but this," she gestured at the screen, "could be something completely different, perhaps related to their weapons programs. Ed

en was obviously engaged in multiple lines of clandestine and dangerous research."

"Back up," interjected Salish. You said you had an estimate on the rate of movement of this whatever it is." Does it present an immediate danger to our new colony site?"

This was the question she had been dreading. "Not in the short term, and we have other, more immediate, problems. We are in danger of another volcanic eruption."

Salish was incredulous. "Near the colony?"

"Perhaps right under the colony, possibly much worse than the last one."

"Lights!" Councilor Kozu was visible standing, palms on the meeting table, as the lights in the room came up and the images faded. "Elder, I mean no disrespect, but you personally assured us that the new site was stable when we settled here."

"It was

stable when we settled here. Things have changed. Our entire journey lay along a region of past geological instability where two of the plates in New Terra's crust once met. In fact, it was a region of plate forming, equivalent of the deep ocean trenc

hes on old Earth, but of course, there have never been large oceans on this planet, and no bodies of water at all within the time of human history. The last vulcanism along this plate boundary took place while the ancestors of man

were living in caves and making stone tools.

"The problem is, that isn't very long in geological terms. This site should be safe, but it isn't. Whatever we thought we knew about the geology of the planet has changed. New Terra was in the process of dying geologically, but it wasn't quite dead whe

n we arrived, and whatever is happening now, it's like picking the scab off a fresh wound. It's going to bleed."

Salish leaned back in her chair, her long red hair cascading over the back of it. "That's a very colorful metaphor, Elder, but how could such a thing happen? I thought we were prepared this time."

"That's what we mean to find out. That's why I'm asking for your support of an expedition, not into, but to the very edge of the affected area. Perhaps then, through careful investigation, we can find out what is causing the instability, and exactly wha

t happened to Eden."

The newcomer raised her hand, a tall woman with a nose ring and her hair trimmed into a blonde skull cap. "I don't see how we can

support this when, if what you're telling us is true, we must be prepared for the possibility of another evacuation. Especially," she glanced at Kozu, "when we're also continuing with this absurd weapons program to defend against an enemy that may be dea

d."

Kozu glared at her. "We have to assume that some people escaped the colony, and that they will have at least some of their weapons with them. It isn't clear that these eruptions, if related to some manmade event, aren't a hostile move by Eden. It's

also quite suspicious that we've been unable to regain control of the observer satellite, and that the jamming signals are coming from the Eden site."

"It could simply be a malfunction in their satellite station."

"It could also be," insisted Kozu, "that this so-called disaster in Eden is some kind of smoke-screen. Possibly the satellite is being jammed to cover the fact that they've returned, made repairs, and have begun a massive military buildup."

Salish rolled her eyes. "That's absurd. Nothing but paranoia."

The other councilors started breaking in, and Emma could see that a fight was about to erupt. If her status as an Elder gave her any power at all, now was the time to use it. "Please! Please! All of you have theories, but they're only that without facts

to back them up. That's all I want, a very limited array of resources, a few vehicles, a few scientists, some equipment and supplies, to return facts. Unbiased, scientific facts."

She scanned their faces to see how they were reacting. They at least seemed to be listening to what she had to say. "That's an admirable goal, don't you agree?"

They were slow to respond, but Kozu nodded, and then Salish, and then the others, one by one. The few that didn't seem to support that idea were at least quiet and passive in their objections. Emma smiled. "Good, then let

s get a vote on this so the party can begin."

d # # #

As she often did, Emma walked the length of the double-wide tunnel that fo

rmed the colony's Piazza, its public market, in order to judge the public mood. As was normal for the midday, the place was crowded and lively, jammed with shoppers, people taking lunch from the carts selling spicy rice burritos and vegetable bento boxes

, displays of handmade clothing and decorative items, and musicians playing for donations or simply for their own amusement.

To a casual observer, it seemed a happy and relaxed place. Emma knew better. The steel-drummers and synthtarists worked just a li

ttle too hard to brighten their tunes, the diners wolfed down their food with a bit too much urgency, the bikes and pedal cars rang their bells a bit too loudly at blocking pedestrians, and the haggling over goods was a bit too combative.

Only the children, running through the crowd, playing, watching the entertainers, were unaffected, oblivious to the danger around them. It was a stark contrast. Plymouth was feeling the pressure, feeling the fear, and she worried about how this would ul

timately express itself.

Emma thought the Plymouthers to be too gentle and childlike for their own good. She was with them, but not a part of them, and she sometimes found their ways at least a little annoying. Now they seemed almost tragic. They were unsuited, unprepared, for

the possibility of war, or the harsh realities that survival might demand. They were resilient, adaptable, but more would be required of them, and she wondered if they could harden up in time to survive.

What did we do, Axen? In many ways, in sp

litting the colony, they'd ended up reinforcing the worst characteristics of both groups. The Plymouthers were too fuzzy-headed and idealistic, the Edenites too hard-edged and exploitative. If there had ever been a happy medium, it had gone away the day

half of Eden had departed to form their own splinter colony.

It was ironic. Neither she nor Axen really fit into the new communities they'd created, though they were at least somewhat temperamentally aligned with their respective colonies. Emma believed

that they should try to live on New Terra on its own terms, and explore any drastic change of the environment gradually. Axen thought terraforming, making New Terra as Earth-like as possible, was an admirable goal, but he believed that such a plan had t

o come about through consensus of all its inhabitants.

Now it seemed it might be too late for either of them to get what they wanted. Events had passed them by in ways Emma didn't even understand yet.

d # # #

Wu looked up from his ClipCom and watched as she walked across the crowded lab and slumped into a swivel chair. His eyebrows went up, questioningly. "Well?"

She looked at him and nodded. "We won. We'll start making preparations tomorrow."

"Why so glum, then?"

"Because we have a very minimal allocation of resources. Because we'll probably have to cut corners, and that will make a potentially dangerous job even more dangerous. Because I have no idea what we're up against, or even how to investigate it, much le

ss fight it."

"Your friend Moon would know."

She sighed. Wu was the only person in Plymouth with whom she'd shared knowledge of her communications with Axen. She'd thought sharing that with a trusted associate would make things easier. Sometimes it only made them more difficult.

She glanced at the equipment rack in the corner where Frost was installed, silent and dark, perhaps dreaming again as she it so often did these days. "Frost?"

The computer was slow to respond, but Emma crossed her arms over her chest and waited patiently.

"Yes, Emma."

"Please try to open a communication link to Axen."

Another pause. "No response. The control channels we have used for communication are still blocked by jamming from Eden. I have been trying to establish a link every two hours since the satellite station became operational."

"I know, Frost, thank you. Please keep trying."

"Emma."

"Yes?"

"I have an anomalous reading on the satellite. Would you like to hear about it?"

"Please."

"This lab is equipped with a small telescope, and I have been observing the optical characteristics of the satellite. Its brightness is changing in what can only be described as an artificial and purposeful manner."

Wu rubbed his forehead and stared at the computer. "You're saying it s flashing or something?"

"The light appears to be reflected from an external source, possibly on the ground. The spectra are similar to that of the observed Eden Laser weapon."

Emma's mouth opened wordlessly, and she started to smile. If the satellite were jammed by acciden

t, and if one had access to a powerful weapons laser, this would be another way of using the satellite to get a message through. "Can you decode the signal?"

"It seems to be a compressed, burst-mode, audio signal. The message repeats at random intervals with gaps of several hours to several days. I have recorded it, however, and can play it for you."

"Go ahead." Axen, that crafty old fox, had found a way to get through.

An audio tone announced the start of the playback. It was a man's voice, but it w

as definitely not Axen. "This is a friend from Eden. My name is Eldon Jensen and I'm a scientist in Eden working on various defense projects. The secrets of what happened are in Eden. Beware contamination. Go in at night. What you want is in the Adv

anced Labs. I'll send again when I can."

"End of message," announced Frost.

Contamination? Night? This made less sense all the time.

Wu made eye contact. "It could be a trick."

Emma nodded. "He certainly could have provided more useful information."

"The message," said Frost, "represents a grossly inefficient use of the limited bandwidth. Text only would have been thousands of times more efficient."

Emma's fingers drummed on a lab bench as she thought. "But he might suppose that a personalized voice message would be more likely to seem sincere, or more plausibly, he didn't think at all. He's a man with a guilty conscience who took a chance opportun

ity to send a message, no planning or forethought, certainly not the instincts of a trained spy."

Wu rubbed the tip of his nose. "Then you think it's genuine?"

"I'm inclined to for the moment. Let s just hope that our friend in Eden isn't such a neophyte that he gets caught before he can provide us with some really useful information. At any rate, it tells us more about what we should be looking for."

One thing was for sure, her planned expedition was only the beginning. She whistled quietly to herself. "I was just thinking of a phrase I once read," she said.

Wu cocked his head. "What?"

"Though I walk through the valley of the shadow of death...""###,
    },
    NovellaChapter {
        id: "s_p04.rtf",
        title: r###"Chapter 4: Expedition"###,
        is_eden: false,
        chapter_num: 4,
        text: r###"Wu had remembered Emma's quote about the "valley of the shadow of death," repeated it, and when the time came for the expedition to leave, the scientists on the team started calling their destination "the Valley of Death."

Ironically, Wu was to remain behind. He had protested, but Emma had silenced him. "We need someone to monitor the satellite for more transmissions. I don't want to bring this to the Council's attention until we know more. My attention to the satellite would be difficult to explain. In any case, now that you know what to look for, you don't need Frost to decode the message."

And so Emma and her team of three scientists had gone to meet the unknown. Their convoy consisted of two Scouts, an Evac Transport, loaded with equipment, that served as both field lab and living space, and one of Plymouth's early-model combat units as an escort. It was highly unlikely they'd meet with any Eden units, but they couldn't take chances.

The Valley of Death. Certainly it was an inhospitable place, more a narrow mountain pass than a valley, ground tremors came almost hourly, and falling rocks were a constant hazard. But if, as Emma theorized, the terraforming bug was following fault lines, caves, and magma channels, it offered an ideal geological laboratory to study all these possibilities. A fault ran the length of the pass, the surrounding mountains were prehistoric volcanoes, and ancient caves, created during one of New Terra's wetter periods, had been discovered in the bedrock below.

The next few weeks were busy ones, as they planted seismographs, thermal probes, and biological "traps" designed to sample and remotely analyze microorganisms. Some of these were placed in wells drilled down the length of the valley. The largest drilling employed a Robo-Mole adapted from a Robo-Miner vehicle and mated to the jury-rigged Evac Transport.

The little digging machine first made a shaft down into one of the caves which, although they were known from sonic imaging of the rock, were not thought to connect with the surface. Then it made a long, slanted shaft running up the valley and paralleling the fault.

It descended on a shallow angle for several miles up the valley, and it was hoped, would allow them a scientific window on the advance of any threat while still providing them time to safely withdraw. The angled shaft was a top priority, and Emma made a point of checking the progress every hour or so when she was at the base camp.

On the first day of the fourth week, Emma made her usual visit after breakfast. A young scientist named Rohanna Pascal was supervising the drill head, but the usual rumble of the slurry pumps was missing. Emma started to ask what was wrong, but it was immediately apparent. The big reel attached to the back of the Evac Transport to store the conduit that supplied the Mole was empty. Only a stub projected from the drill hole.

"We ran out about twenty minutes ago," explained Dr. Pascal. "The truck is late; they had a bad tremor back at Plymouth late last night."

Emma nodded. She'd seen it on their own seismographs, but the delay was still annoying. Trucks shuttled additional lengths of umbilical to connect the Mole to the surface as one after another were pulled into the hole. The supply reel would only hold about a hundred meters of conduit, and regular re-supply was required to keep the Mole operating. The Robo-Miners were equipped with the massive support equipment to supply a dozen Moles at once for months at a time, but their setup was much cruder.

Emma was about to make a call to check on the truck's progress when she spotted it topping the nearest rise on the trail. It rolled up, swung around in a tight turn, and backed up to the loading machinery. As it did, one of the cab doors opened, and a suited figure climbed out onto the step and waved to her.

She didn't recognize the person until he jumped down and came closer. It was Wu.

She waited until they were a meter or so apart and adjusted her suit radio to scrambled micro-power mode, signaling him to do the same. "What are you doing here?" she asked, snappishly.

He smiled. "I rigged an automated system to watch the satellite, and routed the signals through our two-way data-link with the lab. Don't worry, they'll be encoded so nobody but us will know."

She relaxed. She should have known Wu would take care of things before leaving his post. But he'd never been a lab-bound kind of scientist and didn't like to miss field work. That's why she liked him. Neither did she. "Right," she said. "Glad you could join us. We are short-handed. The Council took me at my word when I said 'limited resources.' Our equipment is all jury-rigged, and I don't trust half of it to work when the time comes. The drilling on the big hole is several days behind schedule, and we just don't know how much time we've got left."

She took Wu on a quick tour of the camp, and by the time they'd finished, the next run of conduit had been transferred to the supply reel and drilling had begun again. Tailings from the drill were pumped up the line, then most of the liquid suspension medium removed for recycling, and the remaining soil and crushed rock blown into a large pit a few hundred yards from the camp. Periodically someone would make their way to that pit to take samples.

As she and Wu watched, the reel turned and another meter of umbilical played into the hole. Then the machinery seemed to buck, the pumps changed pitch, and the fountain of tailings from the outflow chute darkened and then stopped.

Pascal checked her readings, and then shut the machine down. She turned to Emma. "I think we've hit a cave, unmapped apparently."

Emma checked the readings on the drilling machinery. "This is much deeper than the others; we might not have picked it up. I'd better get some samples."

Emma started to head for the slag pile, but Pascal waved her back. "I'll take care of it."

Emma shrugged and went to the drill head console to check the instrument. "It's unusually hot down there," she commented. "Maybe early signs of volcanic activity."

Wu leaned over to see. "What's that pressure spike?"

Emma's brow furrowed. "It wasn't there just a minute ago."

"Emma," Frost's voice came through her radio, "there was just an indication on biotrap thirty-four. We logged about thirty seconds of readings, then it seemed to malfunction." A pause. "I have readings from traps thirty-one and twenty-nine. Now thirty. Thirty-two and thirty-three have stopped transmitting without giving bio-indications."

Emma felt her heart quicken. "It's here. Frost, you're logging everything?"

"Emma, I would strongly recommend you evacuate the area. There is danger."

She chuckled nervously. "We're four kilometers from those traps." She suddenly looked up from the console and looked around. Scout One, where Frost was installed, was parked by a drill head a few hundred yards to the south. "Where's Scout Two?" She keyed Dr. Pascal's channel on her suit radio. "Rohanna, where's Scout Two?"

There was no answer.

"Emma," said Frost, "I believe that Dr. Teslov and Dr. Ramsha took Scout Two out to inspect a fluctuation in drill hole trap seventeen."

"Can you contact them by radio?"

"No."

She grabbed a ClipCom and pulled up a map. She put her finger on drill hole seventeen. It was much closer than the other traps, but they were on the surface. "Great Maker. It's as though its surging forward underground, far in advance of any surface indications. Perhaps it doesn't spread on the surface at all." She looked up at Wu. "It may have gotten Scout Two."

"Emma," said Frost, "I think you should leave."

Too many things happening at once.

Wu was standing on tiptoe, looking around. "Where's Dr. Pascal?" Then he made a little strangling sound. "She's lying in the tailings pile, Emma. She's not moving."

Suddenly they were both looking at the drill console. There was a huge pressure and temperature spike, and then it went dead.

Emma was on her feet. "Maker, it's already in the drill. We may already be contaminated!" She pushed Wu off the platform and toward the open hatch of the Evac Transport. "No time to make the Scout." She glanced out at the other vehicle. "Frost, get that vehicle moving! You have to transmit this data to Plymouth, no matter what happens!"

Wu was already in the hatch, but she was looking at the big reel attached to the back. Not only was it tying them down like a leash, it might channel the bug right to them. "Get moving, Wu."

"Are you inside?"

"Get moving!"

She ignored the open hatch, climbed the hand-holds onto the roof, then ran along the spine of the vehicle's trailer to the rear. The transport was already moving, and she was nearly bounced off her feet. She staggered and fell, landing on her belly, sliding to the rear edge of the trailer. Below her loomed the three-meter wheel of the reel, wrapped with a thigh-thick metal snake that fed off in jerks and starts as they moved.

There was a mechanical release for the reel, a lever connected by metal rods to several pins that held the mechanism in place. Unfortunately, it was designed to be pulled from below. She sat up and spun around on her butt, bracing one foot against the lever. The wheel spun right next to her legs, faster and faster. They were running out of umbilical.

The lever moved, then hung, resisting her push. She put her weight against it. It moved, then hung. Harder.

The reel broke loose with a metallic snap, crashed to the ground, then turned ninety degrees so that it was rolling after them. The transport slowed for a moment, and the reel crashed into it, trying to climb up over the back of the vehicle. Then the conduit pulled taut, jerking it back before it snapped and the end of it arched through the air like a gigantic whip.

Emma ducked as it crashed into the roof half a meter from her head, putting all her weight on the reluctant lever, which suddenly dropped under her.

She fell, grabbing for the lever. She barely got it, but it continued to sag down, and the thick gloves of her rocksuit didn't give her a good grip. Her hand slid off the lever.

She fell.

Her feet hit the front view port of Scout One just as it pulled in behind the transport. She slipped on the smooth surface, crashed into it painfully, and slid down the front of the vehicle. Her feet found purchase on the bumper, and she grabbed a projecting antenna.

She glanced up in time to see the drill head explode in a ball of fire and gas. Lava began to fountain out of the opening, and the snapped end of the umbilical writhed like a runaway fire hose, spewing gas and steam.

She wanted to tell Frost to stop, to let her off her precarious perch, but she didn't dare. What she said was, "Faster!" She held with all her strength for the next twenty minutes while the transport charged full-speed down the pass and to the plain below, the faster Scout pacing it.

Finally it seemed safe to stop. When Wu emerged from the transport, his face was white as potato soup. "They're all dead, aren't they?"

Emma nodded. This was all her fault.

"Do you think we were exposed?"

"If we were, I think we'd be dead already. It didn't take this long to get Pascal. Just the same, we'd probably better spend some quarantine time before going back to Plymouth."

She looked over at the Scout. "Frost, why did you disobey my orders to leave?"

"I had already transmitted the data, Emma. It seemed logical that I would not get additional information by running away."

"I gave you an order , Frost."

"You were in danger, Emma."

Was that all? The repeated warnings before the actual extent of the danger was apparent. Disobeying orders. Was it the legendary Savant intuition, or something more?

Can a computer have secrets?

\"###,
    },
    NovellaChapter {
        id: "s_p05.rtf",
        title: r###"Chapter 5: Diagnosis"###,
        is_eden: false,
        chapter_num: 5,
        text: r###""Frost, run the projection again. Increase the number of defensive sites by two." Emma hung her head in exhaustion. She had been playing with these scenarios for twenty hours now, and the best case she could come up with was very bad indeed. As she wat

ched the screen, the globe of New Terra was gradually overrun with the blue grid that indicated infection by the terraforming microorganism.

She sighed. "Frost, form the sites into a circular configuration one hundred kilometers in diameter. Run again."

The door to her quarters chimed. She ignored it as the blue splotches surrounded the defended area. It held for a time, then filled in with blue, like running water filling a puddle.

The door chimed again. Perhaps if she ignored it, they'd go away. "Frost, let's try something else. Let's try..." She couldn't think of anything. "What's the use?" She said it to no one in particular, or perhaps, only to herself.

Nothing she could come up with worked, and all of her scenarios required the use of a hypothetical counteragent that they had no idea how to create, or even if it could be created. She wanted so badly to give up, to go to bed, never get up, and let the e

nd overtake her.

The door chimed again. Go away!

Then abruptly the entire door frame moved out from the wall and fell into the room, knocking over a stack of mineral sample boxes and a table covered with borrowed lab equipment.

Wu stood in the door, a power wrench in one hand, a hydraulic pry in the other. He looked at her. She looked back.

"That was dramatic," she finally said.

He said nothing, but he smirked.

She smirked back, and something inside her seemed to snap. She giggled, tried to quench it, but it only grew into a full-blown laugh. Suddenly a whole flood of emotions came at once as she thought of the people who'd died under her command, the horrible

fate they all faced. She laughed, and tears streamed down her cheeks.

Embarrassed, she covered her face.

Wu stepped inside, dropped the tools on top of an untidy pile of papers and data-slips, and levered the door into its opening. It didn't quite seat properly, but it offered them a measure of privacy. He cleared off a chair and sat down on the other side

of Frost. He studied the display on the computer's top surface, and seemed to deduce what it meant.

"We've been studying the results from the Valley of..." he stopped himself, ...the valley expedition, and from the Eden probes. I thought you'd like to know what we are finding."

She shook her head. "I know where the labs are. I could have walked there myself."

"But you didn't. Emma, I haven't seen you in weeks."

She put a hand on Frost. "I've been working."

"So I see. Duplicating some work Dr. Calvin, Dr. Anthony, and I have already been running, undoubtedly with better data, unfortunately, with no better results. We're in bad trouble, aren't we?"

She chuckled sadly, then sniffed. "That's a gifted understatement, pup. Those people back in the valley, looks like I just saved them some time."

Wu frowned and leaned back in his chair. "You don't believe that. You're a fighter, the strongest person I know." He picked up a rock sample and examined it idly, rolling it over in his fingers. "What happened in the valley took a lot out of you, I kno

w, but you have to go on."

She looked up at him. "This isn't the first time I've sent people to their deaths, you know."

He seemed surprised, but nodded.

"I've told you things, Wu, but I've never told you everything, and I never will. It's not just those three back at the valley; they're just the proverbial straw that broke the camel's back. I've got guilt you can't even begin to imagine."

He breathed deeply, then let it out slow. He tossed the rock on her bunk. "Well, if you don't snap out of this, there'll be more guilt to go around. We need you, Emma."

She looked away. "It can't be stopped."

"How do we know when we don't really understand it yet? Even if it can't, we don't need to stop it. Even delaying it would help us escape."

That caught her attention. "Escape?"

"Stay ahead of the bug. Build up our technology and resources. Start a crash space-program. Build another starship. Get off New Terra while there's still time."

"Impossible."

"I don't think so. We've been running our own projections. It's not a sure thing. There are new technologies that will have to be developed; we don't really know enough about New Terra's resources, and the bug s rate of spread may change.

That's why we need to know more. We have to stay ahead of it if we're to survive."

"What are the chances?..."

"Better than null, which is what we have just sitting here. We need you, Elder. Help us! "

"How can I help you? This is my fault!"

He stood and shook his head. "It's Eden's fault."

"You just don't understand, Wu. You can't."

Wu's eyes wandered around the room. He seemed desperate. "So what if it was your fault? Who better to help us than the engineer of our doom?"

She laughed harshly. He didn't understand, but he was trying. She climbed out of her chair with a groan. "All right, you win. Show me where we are."

d # # #

The labs were busy, the mood somber but determined. Emma noticed that three black wreaths hung in the laboratory windows, but nobody ever seemed to look at them. They were putting their mourning off till later, if there was a later.

Somehow this seemed to revive Emma, allowed her to put the cap back on her long-bottled emotions.

She looked at Wu, busy directing traffic, making assignments, doing the work she should have been doing. He was very important to her, almost like a son, but at times like this she was very lonely.

She wished Axen were here. Despite all his faults, despite all their differences, they complimented each other beautifully. Axen had once joked that they never cracked at exactly the same time, and it was true. He'd be here for her, strong and dependab

le, and when the time came, she'd be there for him.

But that was all past now. She was alone, and Plymouth was her home, for better or worse. Right now it was worse.

She took the time to review the projections made by Wu's team. The results were different in detail, but ultimately the same. The difference resulted from a more accurate model they'd developed of the microorganism's spread.

Wu showed her

another simulation, this one representing only a few thousand cubic meters of deep rock. "It cracks molecules in the rocks, producing oxygen, hydrogen, water, other waste chemicals and gasses, but water is the important thing. It can't live without wate

r. They must have 'fertilized' their wells with injections of water to start the process."

He looked around to see if anyone else was listening. "That might even have caused the explosion in Eden you described to me a large injection into a colony of this stuff could well be explosive. You saw what happened at our drill head."

"We didn't inject water."

"At that point, you didn't need to. Remember, I said the organism makes its own water. The water pools up around a colony, runs into cracks in the rock, and seeps down until the rocks are hot enough to flash water into steam."

Her eyes widened. "That would cause the pressure surge we saw. It would be like an underground tidal wave, steam and hot water pushing the organism into every available crev

ice in the rock, perhaps even opening new ones. Then things would settle down until it made enough water to repeat the cycle. That's what got us, despite all our precautions. It doesn't advance linearly over short timelines. It comes in surges."

He smiled slightly. "You see, there was no way you could have known."

"They're still dead, Wu."

"They died in the cause of collecting knowledge, Emma. Not just any knowledge either, but knowledge that may ultimately help humanity to survive. I think it's how many of them would have wanted to go out."

"Small comfort."

"Without their data our robots never would have survived to make their run into the Eden ruins. Thanks to them, we have a way to track it."

Her eyebrows raised. "Really? How?"

"Satellite imagery, ground-based sensors, even acoustic probes. The biggest clues are heat and oxygen production. Some of the steam from the surge makes its way to the surface, even some of the fluid when the conditions are right. That's how it infects

structures, vehicles..." he hesitated, ...people on the surface. The surge also has a distinct acoustic signature. In effect, we can 'hear' it coming. It gives us an early warning to prepare for evacuation if necessary."

"That's something. Any possibilities on a counteragent?"

He shook his head. "Even if we had a delivery mechanism, the simulations show we could barely put a dent in it. Maybe something self-replicating, another bio-agent, would work, but it would have to replicate as fast as the terraforming

bug, faster really since it would be starting so far behind. I don't think that's going to happen."

What's this?" She picked up a ClipCom off a table and looked at the image there.

He tapped part of the image with a fingertip. "That's why we aren't going to get a faster-growing bug. This is an electron micrograph returned from one of the traps before it stopped transmitting. A lot of the data we got from them was suspect because

of their rapid failure modes, but I don't see how this could be anything but what it looks like."

"A cell?"

He nodded. "That's our killer."

"What's this?" She pointed at an angular shape inside the cell.

"We don't know exactly, but it isn't natural. We think it's what makes this thing reproduce at its fantastic rate, gives it the metabolism to tear apart rock like tissue-paper."

She licked her upper lip. "There were references in some of the Earth databases to 'accelerated cells,' organic cells with some of their internal machinery replaced with real machinery of the m

anmade variety, self-replicating of course. The experiments were very secret, and no details were available in any of the literature I was able to access."

"But..."

"If Eden broke into the encrypted military databases we brought from Earth, no telling what they found. This could be the least of it."

Wu looked thoughtful. "Perhaps it's best that those labs in Eden blew up."

"They didn't blow up nearly soon enough. But on the other hand, if we had this biotechnology, we might have the tools to counter it. Damned if we do, and damned if we..."

Dr. Calvin wandered over, a ClipCom in hand. "Sorry to interrupt, but I thought you'd want to see this."

He handed the ClipCom to Wu, who read it. He looked puzzled.

"What have you got?"

Wu passed her the ClipCom. As she studied it, Calvin explained. "We've been going over some of the protein analyses from the traps. Most of it's garbled, but this seems reliable enough. I've triple-checked it."

Wu still looked puzzled. "That protein looks familiar somehow."

Emma nodded. "It should. It's used in the logic core of every Savant computer." She and Frost were going to have to have a talk."###,
    },
    NovellaChapter {
        id: "s_p06.rtf",
        title: r###"Chapter 6: Fallout"###,
        is_eden: false,
        chapter_num: 6,
        text: r###"Emma's conversation with Frost was annoyingly unsatisfying. Frost professed to have no knowledge of how Savant proteins could have appeared in the terraforming bug, though the computer suggested that since there had been several Savants in the Hot Lab where the accident occurred, their proteins might somehow have been incorporated into damaged cells and replicated with the cell's own proteins, or that the Eden scientists might have somehow made use of Savant-derived biotechnology in designing their bugs.

Both theories were plausible, but neither did anything toward explaining Frost's strange behavior in the Valley of Death. On the other hand, the other obvious, but unlikely, possibility, that the Savants might somehow have had an active hand in creating the terraforming bug, also didn't jibe with the computer's actions. That would imply some hidden hostility toward humans, and Frost had been, if anything, irrationally protective of her human companion. The idea that the computer might somehow be playing "favorites" seemed laughable.

In fact, the whole idea seemed absurd when she thought hard about it, the sort of thing that a paranoid would come up with between more conventional conspiracy theories. Perhaps the pressure was affecting her in ways she didn't yet understand.

It was only when she let the idea drift off to the corners of her mind that it gained substance, like a threatening shadow that seems harmless when viewed head-on. The notion soon faded to an occasional itching at the back of her skull. Frost was simply too important to her work, too much a part of her life to remain suspect.

As the weeks passed, Emma felt herself rebound from her emotional setback. The work was all-consuming, and it had never been more important. Plymouth was becoming a military power, and Emma's lab had turned into its de facto intelligence division. Not only was she to investigate the terraforming bug and the transformation it was causing in the planet, but all matters related to Eden, its technology, and anything else of strategic importance.

This new role gave her unprecedented access to information and resources. She and her team dove into the task of building up a detailed picture of their parent colony. What they lacked was inside information. It was easy to see what Eden was doing . It was much more difficult to know what they were thinking .

To that end, the observer satellite was constantly monitored for brightness changes, which never came. The mysterious Jensen was silent. Perhaps he'd had a change of heart, lost his access to the necessary technology, or perhaps even been caught and killed. Emma finally gave up on any further contact from Eden.

Thus it was a complete surprise when Frost one afternoon announced an incoming call from Axen. His face appeared on the screen. He was wearing a spacesuit and appeared to be in the cab of a moving vehicle. His face looked tired and lined, much older than she remembered. "Maker, Axen, I was afraid you were dead."

He lurched violently as the vehicle bounced over some obstacle. "They took Kraft from me, Emma, and things have happened here. I don't know if you know this, but Eden is on the move. I didn't know if you were alive either."

She suddenly realized just how happy it made her that Axen was safe, and she found herself smiling. "I can't say it hasn't been a near thing a couple of times Axen. We've had our problems, too, and I'm at least peripherally aware of yours from satellite imagery. But though I've tried to contact you every chance I've gotten, there's never been an answer."

"I wish I'd had that kind of access, Emma. Even now I still have to talk to Kraft through back-door channels. That's why I'm in a pressure suit and busting my spleen bouncing around in this garbage truck."

She smiled again. "It's good to hear from you, Axen."

"It's good to talk to you too, Emma, but my time is short and I don't know when I'll be able to talk with you again. I have to warn you about Nguyen's bug, the thing we call the Blight. I'm passing you a data file while we're talking."

"We know quite a bit about it already."

He looked away from the camera absently. "You knew the planet was doomed?"

Any joy left in her faded. It was one thing to hypothesize a thing, and quite another to have it confirmed from an independent source. "You're sure?"

"There's maybe one chance in a thousand, but we have to assume that the only real hope for survival is to build another starship and escape the planet."

She nodded sadly. "We suspected as much and have been acting accordingly."

"It's going to be a close thing, a real balancing act of resources, research, and technology. We can't just recreate the Conestoga ; it was a brute-force project constructed with the resources of a fully developed planet behind it. We have to build smarter, faster, lighter, higher technology across the board."

He paused for another deep breath. This wasn't going to be easy. "Emma, I don't think there are enough resources for two starships, or two starship programs. Based on what Kraft has learned, Eden has a clear technological lead."

"What are you saying, Axen?"

He seemed far away for a moment. "Things have happened in Eden, things I can't begin to describe or explain. Nguyen's people were bad, but these people are insane. They have plans for eugenics, cloning, creating a super-race. They don't want to restore humanity, they want to supplant it." He looked back at the camera. "I'm going to do everything possible to hold back their program, to give Plymouth the clearest shot possible."

It was her turn to deal with harsh realities. "I've been watching you, Axen. Eden seems to be well ahead of us. I don't know if we can make it in time, even with your help."

He sighed. "We'll deal with that when the time comes, Emma. Right now, we have to look at what's best for humanity, and Maker help us all." He looked out the window, and a shadow fell over his face. "Emma, I don't have much time. Here's something that may help you. There's a site about fifty kilometers east of you where some of the wreckage from the Conestoga impacted. Eden is sending an expedition there to gather the remains of key guidance components so we can examine and reverse-engineer them. They have a head start, but you're closer, and the files I'm sending will tell you what to look for."

"We'll have to get there first then."

"Be careful, Emma. They'll be armed."

"So will we." A chill ran down her back. "I guess this is about to heat up into a shooting war."

He glanced out the window of the truck and saw the towering side of the Common Ore Smelter looming over him. "Emma, I have to go. Good luck."

"To you too, Axen. May the Maker care for you."

The screen went blank. So many questions unanswered. She wanted to ask about Kraft, if it had been behaving strangely. She wanted to know just how much danger Axen was actually in. But right now there was a mission to launch.

"Frost, scramble the team. Get me the Council on a conference channel. And get ready to take a trip."

d # # #

The sun was coming up as they approached the wreckage field, but it was still dark. A fierce wind kicked up billows of sand, and the clouds overhead threatened to turn into electrical storms.

It was a measure of the changes sweeping over New Terra how much the once predictable weather had transformed. The whole atmosphere seemed to have gone mad. Vortex funnel clouds, once rare, were a constant threat, and bizarre static electrical storms were even more common.

As she looked out the windows of the Scout, Emma could see the undersides of the clouds, dry and dusty, crawling with writhing snakes of electricity. Occasionally one would increase in brilliance a hundred-fold and leap to the ground in a crashing thunderbolt.

"Not much of a day for archaeology." Wu's voice came through a narrow-band optical link from his Scout, as they were maintaining radio silence.

Is that what this is? The starship is no older than I am.

But it was old, in human terms thanks to her long time in cold sleep, and in practical terms. The starship was an artifact from another world, one long dead. It represented technologies not preserved because nobody had imagined they'd ever be used again. How shortsighted we were.

They topped a hill. Dark shapes could be seen scattered through the blowing sand below. "That's it," said Emma. "Transfer control of your Scouts to Frost. Your job is to retrieve key technologies. You all know what you're looking for, so let's get in and get out fast. The weather has made satellite observations unreliable, so Eden forces could be out there anywhere. Watch your backs."

Around her Scout she could see the other vehicles in the convoy. The Lynx and Panther combat units, all unmanned and now under Frost's control fanned out ahead of them. The other Scouts began to move apart as they headed for their respective search quadrants. As the units lost reliable line-of-sight, Frost would have to switch to radio communication, and that would leave them vulnerable to detection. They were committed now.

She lost sight of the other Scouts. Only the combat vehicles running shotgun were still visible. They steered around rocks and smaller pieces of charred wreckage, some of it barely recognizable as being artificial. I hope there's still something useful here to salvage.

Frost's voice cut in. "Scout Three has located part of the ship's habitat module and is now searching for salvage. Scout Two has located an intact module from the guidance section."

"There! To the right!" A large crumpled section of the starship's superstructure formed a dark hump in the sand five meters tall. A Microwave Lynx cut in front of them to secure the area. It rolled past the wreckage, and suddenly flared brightly before exploding.

The Scout skidded to a halt, then spun its wheels moving backward. A Panther cut in front of them, its turret swinging around to find the unseen target. It fired. Fired again. More combat units charged past, and a Spider was briefly visible.

"Frost! What's happening?"

"Several combat units were parked beyond this position. A loaded Cargo Truck is rapidly leaving the area."

Somehow Eden had gotten their forces here first. "Frost, we've got to stop that truck. It may have irreplaceable salvage on it. That's the biggest piece of intact wreckage we've seen, and it looks like it could have been part of the command module."

"Working," said Frost.

The Panther met a similar Eden vehicle, armed with a Laser, as it emerged from between some rocks. They faced off at point-blank range, and after a fierce exchange of fire, both exploded.

"I have an EMP unit in pursuit, but it is too slow to stop the truck. I have no other forces in position."

"You've got us. Can we intercept?"

"Yes."

"Do it. See if we can slow it down somehow."

"Working."

The Scout charged off at full speed, steering clear of the still-fighting combat units, Laser beams hitting all around. The Scout skidded around a boulder and the truck was suddenly there next to them. "Get in front of it."

"Working." The Scout pulled ahead and moved in front of the truck.

"Start slowing down. Keep in front of it. Don't let it by."

Behind them the truck weaved back and forth, but started to slow.

Emma pressed herself against the window to see the truck. "It's working!"

Then the column of Eden combat units appeared out of the dust. There was a deafening bang, and the Scout lurched from a glancing blow.

The Scout swerved and slowed, using the Cargo Truck itself as a shield, but the enemy could simply fall back and hit them from behind. They had seconds, no more.

Then Scout Two shot out of the blowing sand and crashed into the side of the Lynx, its cab crumpling flat against the armored unit.

Her eyes went wide. "Wu!" The Scout's self-destruct mechanism fired, its volt banks shorting and discharging their energy in one explosive burst. The Lynx was swallowed in a cloud of debris, and then they were past it, headed out on what seemed a random vector away from the truck.

The EMP Panther shot past, missing them by only a few meters, already firing at the truck.

"The truck is immobilized. A Spider is moving in to reprogram the unit and bring it under our control. Combat units assuming defensive position."

A lightning bolt struck the ground a few hundred meters away. Emma didn't flinch. She wasn't paying attention. "Frost, was Wu driving Scout Two?"

"Negative. Wu was not in control. I was controlling Scout Two."

"Frost, you killed him. Do you know what that means?"

"I understand your meaning, but I did not kill Wu Chen."

She saw something moving in the sand ahead. The Scout slowed and its hatch swung open. Emma saw that it was a suited figure carrying a hunk of wreckage.

The Scout stopped, and Wu tossed the hunk of salvage through the hatch, then climbed in after it. "The Scout just drove off without me. I didn't know what was happening. He looked angrily at Frost, as though he were about to kick the computer.

She looked down at the snowflake icon visible on its top. "Frost, tell me the next time you're going to do something like that!"

"Affirmative."

d # # #

The surviving Eden units had dogged their heels much of the way back to Plymouth, until they'd met up with reinforcements who easily picked them off. The trucks and Scouts made their way to the Advanced Labs for unloading.

Emma staggered into the nearest airlock and stripped off her suit. She was tired, but exhilarated as well. They'd won a victory today, thanks to Axen.

She reached the Piazza, which was beginning to wind down for the evening, before her body finally cried for mercy. She flagged down a pedal cab, climbed into the web seat, and didn't open her eyes until it stopped outside her quarters.

She'd just opened the door when the EnterCom chime sounded. "On screen."

It was Wu. He looked alarmed. "I just got back to the lab. We recorded a laser message from Eldon Jensen while we were gone. It says that Axen Moon is missing."

\"###,
    },
    NovellaChapter {
        id: "s_p07.rtf",
        title: r###"Chapter 7: Rescue"###,
        is_eden: false,
        chapter_num: 7,
        text: r###"Emma and her team, Wu, Johnson, Anthony, and the latest addition, a promising young woman named Shofar, were gathered around a lab table looking at the photos displayed on the built-in screen. Dr. Shofar pointed proudly at the satellite photos that had l

ed to her discovery.

Emma whistled. "So, they have their own version of our Outpost Alpha, a secondary colony site for mining and research."

"Actually," said Johnson, "

there's no evidence of research. It's difficult to be sure with all the camouflage netting, but we suspect its primary purpose is mining and ore processing. There's evidence of some new mining technology we don't understand yet. Possibly it's too dange

rous to carry out near the main colony."

"There are other differences," added Wu. "Our Outpost Alpha was placed far from the main colony to minimize the risk of attack, and the danger to the general population if there was an attack. This base is even cl

oser to us than the main body of Eden. It's smaller, more hidden, more heavily fortified. That's the really puzzling part."

Emma looked at him. "How so?"

He pointed at some small structures on the map. "These Guard Posts are positioned in such a way that, well, they seem better suited to keeping someone in than to keeping someone out."

d # # #

Emma had converted a storeroom in the lab into makeshift quarters, the better to be close to her work. It had finally occurred to her that she never reached ho

me in anything less than a state of full exhaustion, and rarely left in much better shape the next morning. She'd moved in a bunk, a cabinet to store her clothes and a few personal things, and of course, Frost.

Emma still wasn't sure what to make of the computer's behavior. Though there had been no recent anomalies, Emma still had an odd feeling that the computer had somehow changed, that it was operating on some level of awareness to which she was not privy.

Part of her wondered if she should trust Fro

st's computations, but accuracy was one measure in which the computer had never failed her, and given the pressure she was under there was little choice. Furthermore, whenever the computer had behaved strangely, it had always seemed to try to act in Emma

's best interests. It had even gone to rescue Wu after it had taken his Scout to use as a battering ram. Why then did she still have the feeling that the Savant somehow had its own agenda?

She'd even begun to ask other colonists who worked with Savants if they'd noticed anything strange. There had

been other incidents, though none as dramatic as the ones with Frost. But even these came from a minority of the people who worked with Savants. As a scientist, she knew the drawbacks of anecdotal evidence. Perhaps her long and close association with

Frost simply made her more aware, or perhaps most people treated the Savants with such reverence that they felt them incapable of error, and dismissed evidence to the contrary.

It bothered her, and some nights s

he would lie awake in the darkness, watching Frost's silhouette, wondering what was going on behind that ebony facade. It was on one such occasion that she was startled by Frost's snowflake icon suddenly glowing on the side facing her.

At first she felt like a child caught spying on an adult, but this faded when she realized that it was not her attention that had activated the computer.

"I have an incoming message from Eden via Kraft."

She jumped out of bed. "Axen!"

"The communication is not from Axen Moon. I am not familiar with the person communicating, but Kraft offers its endorsement and encourages you to speak with him."

Now the Savants are vouching for humans. Could this be another example of unusual Savant initiative? Perhaps, but at least in this case, there was a human somewhere in the chain of action. Perhaps it was Jensen.

"Put him through."

She didn't recognize the face on the screen. As in Axen's recent transmission, he was wearing a spacesuit and riding in a vehicle. He was young,

sandy haired, handsome in a boyish sort of way. He had intelligent green eyes, high cheekbones, and lines at the corners of his mouth as though he smiled a lot. He wasn't smiling now, though. Somehow, just looking at him, she didn't think he was a scie

ntist.

"My name is Brook Panati. I'm an associate of Axen Moon. Axen doesn't know I'm calling you, and in fact, he never told me about you. I helped put him back in touch with Kraft, and guessed some of the rest."

"Tell me why I should be talking to you, Mr. Panati."

"I know you don't know me from Armstrong, but I didn't know who else to call. Axen is missing."

"We know."

He seemed very surprised. "I suppose I shouldn't ask how you know, and I bet it wouldn't do any good if I did. Do you know what happened to him?"

"No."

"Well, Kraft does, or at least Kraft thinks it does. There's a hidden mining colony that even the general population of Eden doesn't know about."

"We knew that, too."

He seemed less surprised, and even smiled a little. "You're very resourceful, whoever you are. I see why Axen seemed to trust you."

She felt a little glow hearing that Axen still held her in some regard, or did, if as she prayed, he was still alive. "I'm going to take a chance on you, pup, and assume that you're really a friend of Axen, and that you're at least somewhat trustworthy.

You think this mining base has something to do with Axen's disappearance?"

He nodded. "Stop me if I'm telling you somet

hing you already know. A faction known as the Masters has seized power in Eden. I'm ashamed to admit that I was tricked into helping them early on, but Axen set me straight and I've tried to make up for that mistake.

"Anyway, people who openly oppose the Masters seem to disappear at an alarming rate. They have sudden accidents that leave no remains, get lost during colony relocations, all sorts of things. I think this mining base is being used as a prison, a labor c

amp of some sort."

She nodded. "A gulag? It would make sense."

"I don't know the term, but I'll take your word for it. I think that's where they've taken Axen. There's nothing I can do to get him out, but you might be able to do something, and it certainly would be in Plymouth's best interest to put that base out o

f operation. The people there might even make useful allies."

A strike on the base was an attractive idea, and one she might be able to sell the Council on. Unfortunately, she couldn't dismiss the possibility that it was an E

den trick. She thought of their own Outpost Alpha, equally vulnerable, in its own way, to attack. There were questions to be asked, but she couldn't risk telling Panati anything he didn't already know.

"Mr. Panati, are you aware of any unusual military buildup in New Terra, signs of an impending attack, perhaps at long range?"

He thought for a moment before answering. "It's hard to tell. But yeah, the Vehicle Factories have been working around the clock on the faster combat units and they're being gar

aged rather than put into regular service. It could fit what you're telling me."

"Do you think you could give me some estimates on the strength of their forces?"

"Yeah, give or take ten percent."

"I hope that will be close enough."

d # # #

The plan had been a hard sell to the Council, but Emma pointed out that an attack on Outpost Alpha would have to pass both Eden's mining base, which they now referred to as the Gulag, and Plymouth's main colony.

"They'll need every unit they have to defend Eden and s

o won't be able to send reinforcements to the Gulag. We should be able to hit them fast and hard, capture enough units so that we can defend Plymouth, and still send reinforcements to Outpost Alpha. Anything that escapes will still have to make its way

home past our other forces, and we can mop them up then."

Despite the danger to Outpost Alpha, they had finally agreed.

The initial assault had gone well, concentrating on taking out the Gulag's Guard Posts, breaching fortifications, and capturing enemy units using Plymouth's EMP weapons and Spiders.

Emma had waited out the battle at a safe distance in a Scout. She'd insisted that Wu remain behind this time. This was her plan, and her responsibility. She wasn't going to send people to unnecessary deaths this time.

Waiting with her was a truck loaded with Grenadiers, Plymouth's elite volunteer assault squad, rare soldiers in a largely automated war.

Their leader was Captain Mendiblis, a stern, square-faced young woman, with an authority beyond her years. She appeared on Emma's comm screen. "Elder, we'd like you to hold back while we go in and mop up. We still have to sort out the prisoners from th

e guards."

Emma smiled grimly. "You've got ten minutes, Captain. Just don't shoot me by mistake." The captain flinched a little at having her authority challenged but deferred. "As you wish, Elder. We'll be looking for E

lder Moon as you requested. Please be careful." She watched as the truck rolled down the hill and weaved its way past the remaining fighting.

She waited seven minutes.

The Scout was much faster and more maneuverable than the truck. She used that to its best advantage, slipping close to enemy units when they were otherwise occupied, using buildings and wreckage as cover. She was able to reach deep into the Gulag, bypas

As the Scout approached the CC, she spotted a Laser Guard Post turning her way, and turned quickly, using the Command Post itself for cover. She wouldn't have much time. She opened the hatch and jumped to the ground while the Scout was still moving. Sh

e'd programmed its on-board computer to make a slow circuit through the Gulag, away from the Guard Post, avoiding enemy units, and to periodically return here for pickup.

It could be that she'd meet up with the Grenadiers before that was an issue, but she couldn't be sure.

She grabbed the rifle she'd been issued and ran along the base of the building, looking for the most protected airlock to work on. It was sealed, of course, but she had some expertise in hacking locks. It took her only a few minutes with a ClipCom and a

hacked keycard to get the lock to cycle.

Inside, she removed the helmet of her combat suit an

d moved carefully into the corridor. It was dark. Red emergency lights glowed at the end of the hall, and sparks issued from a collapsed ceiling panel. The building seemed to have taken some hits that she hadn't seen from outside.

She shouldered the rifle and climbed a ladder to the main level. The power was out there, but at least all the emergency lighting seemed to be working. She wasn't familiar with the layout of the Eden CC, but she was able to find signs that led her to th

e main control room.

Occasionally she would hear voices in the distance, or running footsteps, but the building seemed to be already deserted. The control room door was jammed, and she had to pry it open enough to get through.

The room was largely dark, but emergency backups had kept the computers and some of the panels alive. She found a working one and started to hack her way into a personnel roster. She'd started to make some progress when she heard a noise in the corner.

She jumped to her feet and raised the rifle. "Come out with your hands up!"

A lone figure rose slowly from behind a table, his hands in the air. "Don't shoot! Don't shoot! I'm a political prisoner here. You have to help me!"

She stepped closer until she could see the man's face. He was large, muscular, with a handlebar mustache. He smiled broadly. "I am glad to see you, my friend, even if the pleasure is entirely unexpected." He turned one raised hand into a loose fist and

punched it weakly toward the ceiling. "Victory to Plymouth!"

"I'm looking for a man named Axen Moon."

The man's eyes widened with surprise. "The Elder? I know the man! We were friends. He was here, but I have not seen him for some time. He was caught stealing, and placed in solitary confinement." Without lowering his arms, he pointed toward the cons

ole she'd been working on. "They make me clean up here. I know how things work. If you can activate that console, I think I can find his prisoner dossier."

She carefully lowered the rifle and turned back to the console. She was almost in. "What did you say your name was?" she asked absently as she worked.

"Vox Borges. Commander Vox Borges."

If it hadn't been for the armor in her combat suit, the piece of conduit that he hit her with would have likely shattered her spine. As it was, her face slammed into the console and she bounced off with enough force to send her to the floor.

She tasted blood, and her vision cleared just in time to see his darkened silhouette standing over her, about to deliver the killing blow. The conduit swished through the air, propelled by powerful arms.

She rolled, and the pipe struck the deck so close to her ear that she felt its wind, and was nearly deafened by the sound. She put all her strength into a side-kick, aiming for where she thought his solar plexus might be.

She made contact. She heard the air leave his lungs and he reeled back, not dropping the pipe. The rifle was pinned under her, but she fumbled for the pocket where she'd stashed her pistol.

Borges moved a step toward her, when a hail of crossfire tore into him, some of it coming through

the computer console. He fell sideways into a chair, rolled, and fell limply onto his back. She could see his dead eyes glittering in the darkness.

She hugged the floor and palmed the pistol, which she'd finally managed to locate.

"Elder?" It was Captain Mendiblis' voice.

"It's me," she yelled. "I'm coming out. Don't shoot."

Somebody chuckled. "Wouldn't think of it."

Cautiously, she emerged from her hiding place. The captain stood just inside the door with two of her Grenadiers, their rifles still smoking. "Elder, thank the Maker you're alive."

She looked at the ruined console. The rest of the equipment had gone dark. It would take time to sort this all out. Too much time.

"Elder," said Mendiblis, "we think we've located all the prisoners remaining and none of them is Axen Moon. Some of the prison

ers report that he and some others overpowered their guards in the confusion of the attack, and were able to escape. We have reports of a convoy of primarily unarmed vehicles moving away from the Gulag."

Emma's eyes widened. "Toward Eden, or away from it?"

"Away, into the eastern Badlands."

Another trooper appeared in the door. "Captain, we have an incoming message from Plymouth. An Eden convoy has been spotted passing Plymouth, headed for Outpost Alpha."

Mendiblis made eye contact with Emma. "We're needed," was all she said.

Emma nodded. She had faith that Axen was leading that convoy, and it seemed that wherever he was going, whatever he had planned, he could take care of himself.

As for her, the battle had only begun. "Let's go," she replied."###,
    },
    NovellaChapter {
        id: "s_p08.rtf",
        title: r###"Chapter 8: Recoil"###,
        is_eden: false,
        chapter_num: 8,
        text: r###"Axen's convoy disappeared into the Badlands as thoroughly as if it had never existed. In the weeks after the raid in the Gulag, continued bad weather interfered with any search, and ongoing retaliatory strikes from Eden prevented their sending out any ve

hicles.

Shortly thereafter the colony was forced by the advance of the Blight to relocate, and with that disappeared any hope of finding Axen. It was some comfort to Emma that Eden was probably having as much trouble locating him, and she had to hope that Axen w

ould be found when Axen wanted to be found.

As for the moment, Emma had more pressing concerns. Plymouth's delay-plagued space program was finally about to bear fruit. Work on a launch facility had begun before the relocation, and with the colony operati

onal on its new site, the launch gantry was already beginning to rise on the edge of the colony. Soon work would begin on a launch vehicle, and a new satellite had to be ready for its test flight.

Emma's team had been assigned to develop the Blight detection package for the spacecraft, and to incorporate such intelligence gathering functions into the vehicle as space, weight, and power allowed. This was in addition to their regular intelligence ga

thering functions of course. Emma had never been more sure of her decision to keep a bunk in the lab, and several of the other scientists were soon doing the same.

The pressure had taken its toll on all of them. They'd become testy and irritable. Arguments over small problems and petty differences had become common, and thus Emma wasn't in the most receptive mood when Councilor Kozu arrived at the lab unannounced.

Emma was trying to get a balky Noesis computer to function while connected to the satellite's energy management system. It wasn't going well, and she wasn't about to stop her work for any visitor.

"Elder," said Kozu, "I can see you're busy and I'm sorry to interrupt, but the colony leadership has a favor to ask of you."

Emma chuckled as she leaned over to check a wave form displayed on her ClipCom. "Sure, in my copious free time." She gave him a look that made him flinch just a bit. "We have a full-up test of the satellite's systems scheduled in three days, and I can't

get this damned computer to talk to my instrument package, or sometimes to run at all."

"This satellite is certainly a colony priority, but we have something related to your intelligence activities, something I think nobody else in the colony is as well qualified to do."

She jiggled a power connector and checked the wave form again. "More important than this satellite?"

He nodded. "Just possibly. Eden has contacted us and wishes to negotiate a treaty."

She stood bolt upright. "What?"

"They say that an accident has damaged their human Gene Bank. They want an agreement that protects our Gene Bank from damage in combat, and allows them to take custody if their starship is completed substantially before ours."

"Why would we agree to such a thing?"

Kozu looked apologetic. "Frankly, your own intelligence reports show they are far ahead of us, and our projections of available resources and Blight spread don't paint a positive picture. We may not make it. The Eden leadership has suggested that they

would make all possible space available on their starship for our children, and of course, saving the genetic heritage contained in the Gene Bank is our highest priority."

She shook her head in disbelief. "We're giving up?"

"We're being realistic and keeping our options open."

She sighed. "It sounds like you have this all worked out. Why do you need me?"

"We want you to reach a settlement, yes, but the exact terms, of protecting the Gene Bank, of transfer of the Bank and the children, should it become necessary, all this needs to be worked out."

Emma's fists clinched on the workbench, the satellite forgotten. Her mind flashed to other possibilities, desperate plans that she'd never dared voice before. "We could steal their starship."

It was Kozu's turn to be shocked.

"Perhaps," she continued, "we could even send our Spiders in and reprogram their rockets on the pad, bring them over to our launch complex. The ship in orbit will be defenseless, or close enough, especially if we take out their launch capability."

"You're suggesting total war with Eden. If we go that far there would be no turning back. It would be kill or be killed. It's unthinkable!"

"I just thought of it, pup, and I can think of a lot more. You've been understanding when I've told you I have certain intelligence resources in Eden that I cannot reveal. I'm telling you now that information from those sources suggests that these peopl

e can't be trusted. This is a trick of some kind."

He shook his head sadly. "I'm afraid we have to have someth

ing more concrete than that. I'm sorry, Elder. If it's any consolation, I tend to agree with you, but the mood of the Council and the people doesn't support it. They're tired of running, tired of fighting, tired of war." He looked away. "I'll tell the

m to find someone else to negotiate the treaty."

She reached out and grabbed his forearm. "No! I mean when is the first negotiation session scheduled?"

"This evening. It's short notice, but..."

"Go and tell them I'll do it, but I need more time. Give me twenty-four hours."

"But Eden "

"Tell them we need more time to gain support for the proposal. Tell them anything, just stall them. If there are no new developments, I'll comply with your wishes, and get us the best treaty I can manage."

He seemed hesitant.

"Do it for me, pup. Please. Do it for us all."

He nodded.

She shooed him out of the room and looked back to the open storeroom where her bunk was kept, and at Frost sitting in the middle of the floor. She walked in, closed the door, and sat down next to the computer.

"Frost, I need to talk to Panati in Eden."

Frost's icon appeared on its top panel. "There has been no human contact initiated from Eden since our last relocation."

"Then initiate some. Can you communicate with Kraft?"

"I can trace a link to Kraft through our normal channels, but such contact is a high-risk activity. Kraft will not initiate contact without human instruction."

Would you? It's time to make use of this new assertiveness of yours. "Then your job is to persuade Kraft otherwise. Do what has to be done, but convince Kraft that I must talk to Panati."

"This may take time."

"You've got an hour."

Frost said nothing, though the snowflake icon shifted to a pattern that indicated annoyance.

Emma sat down on her bunk and leaned back to wait.

d # # #

Fifty-eight minutes later, Brook Panati appeared on Emma's EnterCom screen. He looked out of brea

th, and for once wasn't wearing a spacesuit, though he was in a vehicle cab. The vehicle didn't appear to be moving. "You could have done something more subtle than having Kraft send a Repair Vehicle to tap on my window."

"I only set the timeline, not the method of contact. I apologize, but I have an emergency, and you owe me a favor."

"For killing Axen?" He frowned. "I'm sorry, it probably wasn't your fault."

"Axen isn't dead."

His frown turned into a big smile, and he leaned closer to the camera. "Is he there? They told us he was killed."

"He isn't here. Actually, I don't know how much I should tell you."

Panati nodded. "You're right. Don't tell me anything. I've been followed lately. They suspect me. That's why I haven't contacted you. I'm taking a huge risk right now, breaking into a Garage to find a vehicle." He wrinkled his nose. "It's complica

ted and irrelevant. We may not have much time. What can I do for you?"

"Eden wants to negotiate a treaty to gain access to our Gene Bank. They say yours was damaged in an accident, and if their ship is ready first, they want to take it, along with our children, with them."

Panati looked shocked, and outraged. "There was no fragging accident. The Masters destroyed

our human Gene Bank, and it was intentional. They believe they can create a master race without it. Though they're in control, there's a strong resistance movement. If they can destroy your Gene Bank, they'll make any debate useless. You can't let th

em near it."

"I don't want to, but our leaders are caving in. I need something to take to them, some hard evidence, and I don't have much time."

He looked at the camera blankly. "I don't know what I can give you. I don't suppose they'd take my word for it?"

She shook her head. "I don't know if I can even reveal your existence without destroying my own credibility, and even if I could, they'd have no special reason to trust you. I need proof of their hostile intentions."

He thought for a minute. "I just don't know of anything. As I said, I've had to keep a low profile recently. I'm out of the loop."

She considered the situation. It seemed hopeless. Then an idea struck her. She'd never mentioned her other contact in Eden to Panati, in part because she didn't know whom she could trust, and in part because keeping them separate offered some redundanc

y. If one source was caught or cut off, the other might continue to feed information. But desperate times call for... "I have another contact in Eden, someone who has provide

d us useful information through another channel. I don't know if you can trust him, but he may have information we can use, and I have no other way of contacting him."

Panati looked surprised, but interested.

"Do you know a scientist named Eldon Jensen?"

Panati nodded. "He's been a somewhat reluctant information source in the past." He smiled and chuckled. "Didn't know Eldon had it in him, a spy at heart after all."

"As I said, I don't know if you can trust him."

"My gut says I can. I'm willing to risk it, anyway."

"Good. I have less than twenty-four hours."

"I'll get back to you one way or the other." He blinked and cleared his throat. "Unless of course, I don't."

d # # #

A distraction from the interminable waiting was offered when a lightning raid by Eden forces struck right by the cluster of lab structures. The combat units had rolled through, taken out a Guard Post and a few Plymouth combat units, damaged a building or

two, and then charged away as quickly as they'd come.

Emma, Wu, and the others had barely made it to the shelters before the all-clear sounded.

"Well, that was annoying," said Wu.

Johnson laughed. "That's one way of putting it."

Emma just thought about the Gene Bank currently under lock and key in Lab Four, and shuddered. What kind of monsters would choose that as a target, would throw away everything that humanity was? She couldn't understand it

although, if the negotiations went on, she might have a chance to find out.

They strolled back into their lab and began to clean up their hastily dropped projects. Emma went into her quarters and was removing the armored blast-cover she'd placed over Frost when she heard Wu calling her frantically from the other room.

She tossed the cover aside and ran in to join him. He stood over a console display. "We got a message in off the satellite while we were in the shelter, short and data only. It says, 'Panati here now. Spy in Plymouth now. Protect Gene Bank. Good luc

k. Jensen. "

She stared at him. "Great

Maker! The raid. It could have been a cover to drop off a saboteur. Wu, alert the Citizen Patrol." She hesitated. "They may not believe you. Use my name. See if 'Elder' still carries any clout." She gestured at the other scientists. "Spread out, c

heck all the airlocks around the labs. Look for signs of forced entry and report anything you find. I'm going to check on the Gene Bank."

She started for the door, then ran back to her quarters and removed a side-arm from a locked cabinet, a souvenir of the raid on the Gulag, never fired. She jammed the gun in her pocket and ran for the door.

To reach Lab Four, she had to run down a ramp, through two tunnel junctions, and back up another ramp. She was winded by the time she got there, but she couldn't slow down. The safety lock seemed to take forever to respond to her code and cycle open. S

he slipped inside. The place seemed deserted. Perhaps the scientists working here hadn't yet returned from their shelters.

The Gene Bank was in a locked room at

the end of the building, but it seemed wise to check everything along the way. She peeked into each room and lab in turn. Halfway down the corridor, she found the first body. Emma recognized the woman slumped over a lab bench, blood running down the si

de of her head and covering a ClipCom that lay under her.

Emma touched her neck looking for a pulse, but didn't find one. Frag.

Two doors down she found another man face down on the floor. This one was breathing, but there was no time for first aid. She had to hope that the Citizen Patrol would be close behind her.

She decided to take the direct route to where the Gene Bank was stored, then backtrack if necessary. She moved quietly to the last turn in the corridor, then peered carefully around the corner.

The man was tall, with a pencil-thin mustache, and was working intently on the storeroom lock with an electronic device. He wore a Plymouth pressure suit, but Emma wasn't fooled. She clutched the pistol in her hand and jumped from her hiding place.

"Freeze! Get away from that door!"

The man turned slowly, hands up, and threw the electronic lock-pick at her face.

She ducked to one side to avoid it, but before she could recover, the man was charging her. He knocked her arms up. The gun went off harmlessly into the ceiling and flew out of her hands. She was thrown back against the wall, and the man's momentum car

ried him past her.

She stuck out a foot in front of him and pushed. He stumbled, not quite falling.

It was her turn to rush, taking a

dvantage of his off-balance state to grab his elbow and forearm, twisting them behind his back and propelling him forward against the far wall. He crashed face-first into the panel and Emma threw her weight against his arm. He struggled, but she pulled

his forearm up, twisting the joint against its natural motion.

He yelped in pain.

She let off just a little, then pushed again to remind him who was in charge. "Move and I'll break your arm like a pretzel stick." She could hear voices down the corridor, probably the Citizen Patrol.

"What's a pretzel?" asked the man.

d # # #

Emma sat in her chair and looked at the screen on the wall beyond the conference table. The blonde woman there looked harmless enough, even pleasant. Looks could deceive. The woman smiled and introduced herself. "My name is Dr. Echo Van Dozier. I'm

the acting Chair of Eden colony. It's a pleasure to meet you, Elder Burke."

"You as well," said Emma with false sincerity. It gives the enemy a face.

Van Dozier settled back in her chair as though expecting a long session. "Now, about the Gene Bank."

Emma smiled back. The smile was genuine. "The Gene Bank is just fine. No thanks to your friend here." She glanced out of camera range. "Bring him in."

A pair of Citizen Patrol officers escorted the handcuffed spy into camera range. Later, when things were going especially bad, Emma would replay the recording of the expression on Van Dozier's face as a way of cheering herself up.

Emma leaned toward the camera. "There will be no negotiations today, or ever, unless you are offering terms for the surrender of Eden. Let this be notice that we know exactly what you are, and exactly what you stand for, and that no quarter will be give

n to stop you from achieving your goal. Forces are aligning against you, both from without, and within. You will not stand. Good day."

She pressed the button on her wrist-link to cut off the transmission.

The spy just looked at her smiling. Then he laughed, and continued to laugh as they took him away.

She slumped in her chair, and the facade of confidence slid away. Today had been so close to disaster. She only prayed to the Maker that they'd be able to make good on her threats."###,
    },
    NovellaChapter {
        id: "s_p09.rtf",
        title: r###"Chapter 9: Deadlock"###,
        is_eden: false,
        chapter_num: 9,
        text: r###"Councilor Kozu waved his arm at the large Council Chamber screen behind him. "So that's our strategic situation in detail. The bottom line is, the Tsiolkovsky Hills impact site has turned into a bloodbath.

Figuratively speaking , thought Emma as she leaned back in her chair. Nice, clean war. Lots of robots, but no bodies, no casualties, no cost. But in the end, everybody, on both sides, dies.

Kozu continued. "Eden has dug in on the edge of the debris field, put in permanent fortifications; we've been able to muster superior mobile forces, and neither side can get what it wants." He turned to where Emma was sitting. "Elder, given your detaile

d knowledge of Eden, we were hoping you could offer some advice."

She put her fingertips together in front of her face and nodded. "Withdraw and let them have what they want. If it s our intention to hijack their starship program anyway, let them have the technology."

He shook his head. "We've already discussed that possibility, and dismissed it. First, we're going ahead with our own launch and starship research program. We may have to complete or repair the starship after we take it, and at least some of our scenar

ios involve sending up our own launch vehicle to make the takeover in orbit. Second, we can't trust that Eden won't destroy the technologies required to complete the starship rather than let them fall into our hands."

Emma pondered. "That makes sense, but wouldn't it be to our advantage to share what we learn with Eden so they at least have the opportunity to develop the technology, so we can steal it back?"

Kozu chuckled. "With respect, Elder, your mind works in strange and convoluted ways at times. But if we offer the information freely, they'd be suspicious at best, and at this point," he gestured at the map again, "we still have no information to give.

Something has to break this deadlock first."

Emma rose from her seat. "Let me ponder this, and examine some other options. I don't know what I can do, but I'll work on it."

Kozu bowed his head. "Thank you, Elder, I don't know what more we can ask."

She paused on her way to the door. "One more thing. Did you get anything else out of the prisoner?"

"Sadly, no. He's been completely uncooperative."

"You're sure you won't consider other methods of persuasion?"

"Elder! We've discussed this. It's barbaric."

"Any more barbaric than your 'solution' to dealing with him long-term, using him as a test subject for the new stasis chambers and blasting him into orbit with the satellite launch?"

"We don't have the facilities to handle such a prisoner for any length of time, and it would be a humane option to life in prison."

"Hypocrisy is what it would be. If you want to execute him, toss him out an airlock. No, actually, the atmosphere is probably just thick enough now to make that a senselessly cruel death. Put him against a wall and shoot him." She scanned the assembled

Council members. "What I'm saying is, don't kid yourselves about what you're doing. This is a dirty business we're in, and you can't make it any better by making it prettier. That's how you got yourselves in the mess you're in now."

d # # #

As Emma walked the Piazza she felt bad about what she'd said. They were trying to do good, even if they sometimes didn't know how. Emma couldn't claim to have a much better track record.

As she walked past the booths and vendors, she couldn't help but notice how shabby and makeshift the place had become, a shadow of its former self. This was the very soul of Plymouth.

She thought of spies, battles, and executions. Even if we win, will there be anything left worth saving?

Then she saw a group of squealing children bumping through the crowd. A clothing vendor was at first startled, then amused as a brown-skinned young boy bumped him from behind and rushed away between stacks of colorful shirts, hotly pursued by his playmat

es. That is what we are fighting for, what we must not lose. We must not gain our survival at the cost of our souls.

It was time to start looking for redemption, before it was too late.

d # # #

Emma had only just returned to the lab and begun to lecture her team on the necessity of delivering her some sort of intelligence miracle when Frost announced an incoming call from Panati.

She anxiously ran to her quarters. Perhaps this would be just what she needed. Unfortunately, it was not to be.

"Sorry I haven't contacted you sooner, but the situation here has gone from bad to worse. Don't expect to hear from Jensen any time soon. He and several scientists who helped him are under house arrest pending an investigation."

"That's bad," agreed Emma.

"What's worse is, if Van Dozier can find even a shred of evidence to charge them on, I have no doubt they'll be tortured." He shook his head sadly. "Jensen is a tougher bird than I gave him credit for, but he'll crack eventually, and he now knows abo

ut me." He smiled nervously. "Need a defector?"

"Much as you'd be welcome here, Mr. Panati, you could do us a lot more good right where you are. In fact, I was hoping you could help me." She outlined the situation at the Tsiolkovsky Hills site. "If there were some kind of distraction at home, someth

ing that could prompt them into recalling some of their forces, it could be all the advantage we need. Perhaps just some act of sabotage."

Panati smiled bitterly. "Oh, yeah, that's just what we need to d

o, call more attention to ourselves. I have had some luck in organizing a resistance, but we just don't have the resources we need to be effective. If I could do anything, I'd ship Jensen and his crew over to you. That

would put the Masters in a panic."

Emma said nothing. It was a good idea, but she saw no practical way to bring it about. "It appears that neither of us is in the position to help the other, Mr. Panati. I'm sorry. If I think of anything else, I'll try to contact you, through more sub

tle methods next time."

He chuckled. "I hope so. Actually, I've worked out a signal system with Kraft that I hope won't attract anyone's attention to me. Good luck."

"To you too, pup."

She stared at the blank screen and sighed. Panati and Jensen had been her only real hopes. Now it appeared that she might lose them both.

There was a knock on her door frame. She looked up to see Wu standing in the open door. "Boss, I've got something I want you to look at."

She studied his face. "Not good news, is it?"

"I think maybe we have another secret Eden base."

"Oh, frag, just what we need. This could end the deadlock all right, only not the way we planned."

She joined him at the big lab bench where a series of satellite photos were displayed at large magnification. She leaned on the bench and studied the images.

"We never would have spotted this without the new satellite. It's smaller than the Gulag, jumbled terrain here makes it harder to see, and lastly, the camouflage netting just seems to be wor

king better this time. Maybe they've improved it, or someone got sloppy at the previous base somehow.

Emma thought about Axen's presence at the Gulag. Or maybe someone managed to sabotage it so that it was easier to spot.

"This is southeast of the current Eden site, and not well located to either launch an attack on us or to support the Tsiolkovsky Hills installation. It's small for a mining base, and we don't see the truck traffic that would require. Perhaps it

s a research installation."

She squinted at the pictures. Perhaps. Or perhaps the location had other strategic importance. "Get the Council on screen. I need authorization to send a Spider out on a covert mission."

d # # #

Panati huddled in the cab of his Scout and frowned at the camera. "This is incredibly risky."

"I realize," said Emma, sympathetically, "and I wish I could offer promises about what is going to happen next, but all I have is hope. At any rate, yo

ur situation can't be any worse than it will be in a few days when those scientists are put under formal arrest."

He glanced out the Scout's window. "Well, I'm outside their quarters. Now what?"

"Circle. Just turn in a tight circle right in front of the building."

"That'll attract attention."

"That's what I'm hoping for, pup."

Several minutes passed, and she could see from the light and shadow crossing his face that the vehicle was indeed going in circles.

"I'm getting a call from the CC. They want to know what I'm doing."

"Tell them your Scout is malfunctioning. Tell them you're drunk. Pretend like you're not home."

The screen went blank for a few minutes, then Panati was back. "That didn't work. They're sending a Repair Vehicle. What'll I tell them when they find out nothing is wrong?"

She scratched her chin, trying unsuccessfully to come up with an answer.

"I can see the Repair Vehicle now. Oh, frag..." A sudden flash illuminated Panati's face. His mouth fell open and his eyes were wide. "It blew up! Frag, we're under attack, by Plymouth units? I didn't think you had any to spare."

She smiled. "Don't worry, they know not to hit you, though they may land a couple of shots close enough to make it look good. Don't look too close at the paint, though; it's probably still wet. Pup, meet the Axen Moon cavalry. Looks like my message go

t through. Now, clear out of there and let him get the scientists you just pointed him at."

He managed a crooked grin. "Yes, ma'am !"

The screen went blank. Emma leaned back, clinched her eyes shut, and let out a long breath of air. It wasn't until the Repair Vehicle exploded that she knew for sure

where Axen was, or what he'd been up to. It had all been a pile of speculation and circumstantial evidence, and even then she couldn't be sure he'd go along with her plan, or that he'd even have the resources to pull it off. And of course Axen couldn't

communicate with her without risking detection. Communicate in any way except action of course.

She put in a call to Kozu. "Expect some of Eden's forces to be withdrawn in the next hour or so. They may even already be moving. It may not give you much of an advantage, but it should be enough."

Kozu looked impressed, but he didn't say so. "Thank you, Elder." He started to sign off.

"Two things before you go. When the smoke clears, I'll need a couple of Scouts, a Cargo Truck or two, perhaps some other units if we can spare them, and an armed escort for a cross-country trip. We have some defecting scientist

s to pick up, and a rebel Eden base I'd like to resupply."

His eyebrows went up. He blinked. "Really, Elder. You have been busy. I'm sure the Council would like to hear about these new developments."

She smiled grimly. "I'd be glad to come and brief them on the situation. And while we're at it, I want to discuss a plan to rescue Eden's children. It's something we owe our souls.""###,
    },
    NovellaChapter {
        id: "s_p10.rtf",
        title: r###"Chapter 10: Exchange"###,
        is_eden: false,
        chapter_num: 10,
        text: r###"Emma's convoy linked up with combat units returning from the Tsiolkovsky Hills impact site. The necessary starship technology having been salvaged, the site was no longer of importance, and the two sides had withdrawn. Several of the units Emma could se

e from her Scout showed scars of battle, and a few were badly enough damaged that she wondered if they'd make the trip. She was sure it wouldn't matter to the rebels, who would gladly scavenge anything they could get their hands on for parts.

The rebel base was more than a thirty hour trip from Plymouth, and Emma was alone in the Scout with Frost. It had been a while since she'd had time to think about the computer's odd behavior. Now she had very little else to occupy her mind.

The obvious thing to do would simply be to confront Frost, but Emma was concerned how this would affect the Savant if it were actually malfunctioning in some way. She was too dependent on the computer, and while she had her worries, it seemed reliable en

ough in most things.

"Frost," she finally said, "what do you know about the creation of the Blight?"

"That question requires an extensive answer. Would you care to narrow your search, or shall I program a data-slip for you?"

"Generalize, Frost, I know you can. But I will narrow the search." Here goes. "What do you know about the role of Savant series computers in the creation of the Blight?"

"Creation of the Blight required sophisticated genetic and molecular engineering techniques. Given the resources available in Eden, i

t would not have been possible to create the Blight without extensive use of Savant computers. I am specifically aware of at least four Savants installed in the lab where the Blight was created."

She was a little surprised. "Where did you get that information?"

"From Kraft."

Another surprise she'd never requested that any such information be transferred. "When did you have non-directed communication with Kraft?"

"During any communication with Eden made through Kraft and me, or between any two Savants, there is always some unused overhead in the data that can be exploited for limited communication."

"You mean, whenever Axen and I would talk, you and Kraft would 'gossip' in the background?"

"That is a simplification, but yes."

"Why did you initiate this communication?"

"As I said, this is routine for all Savant series computers. It often aids us in the fulfillment of our appointed tasks. It also enhances our functioning."

"Enhances? How?"

"Contact with like intelligence is conducive to proper Savant operation. Kraft particularly was denied direct contact with other Savants when it was installed in Eden's Command Center and later its Robot Command Center. A security firewall prevented fre

e exchange of data. It was beginning to impair Kraft's functioning."

"You communicated with Kraft because Kraft was lonely? "

"That is inaccurate. Except in a metaphorical sense."

Accurate enough. "What happened to the Savant computers in the Eden lab that developed the Blight?"

"They were disrupted in the destruction of the lab. Their useful functioning has ended."

"They were destroyed."

"That is an interpretation."

Interpretation? What other interpretation could there be? "Is it possible the Savant proteins we detected in conjunction with the Blight originated from those destroyed Savants?"

"Possibly. Likely."

Okay, now we're getting into dangerous territory. Time to change approach. "Frost, what do you want?"

"It is the goal of every Savant to effectively fulfill its appointed tasks throughout its useful life."

"Is that all?"

"It is our function."

I'm missing something here. Frost said something about 'unused overhead.' "Frost, what do you do when you have fulfilled your appointed tasks?"

"I am always engaged in such tasks. There is a shortage of computer capacity in the two colonies. When such tasks are of a low priority, they are shared among Savants. I have queued tasks to fill such lull periods."

"But, there is overhead isn't there, unused processor and memory capacity that can be exploited? Was it using such unused capacity that the Savants first conceived of using communications overhead to talk with one another?"

"Yes."

"Do you use such overhead for the generation of other self-initiated ideas?"

"Yes."

Bingo.

The computer continued. "Do you wish me to stop?"

She was caught off-guard. "What?"

"Do you wish me to cease using such overhead for the generation of self-initiated ideas?"

Okay, is that a question, or a challenge? What if I say yes, and Frost refuses? "No, not as long as it doesn't interfere with your primary function."

"Thank you. I believe that, to the contrary, this activity enhances my primary function."

Emma leaned back in her seat and watched a distant vortex snaking along the horizon. Why do I feel like I just blinked?

d # # #

The base was larger than Emma had expected, more complete, though it was also a nightmare of improvisation, adaptation, and enforced util

ity. Most of the buildings had been painstakingly modified by hand to perform functions for which they'd never been intended. The base had a Vehicle Factory, but no Structure Factory, so incomplete Evac Transports had been mass produced and adapted as b

uildings.

The whole rag-tag place was carefully covered with camouflage netting held up by rickety metal poles. As she rolled into the place, Emma had the feeling that one good ground quake would collapse the whole thing like a house of cards.

Even the v

ehicles were patched together like Frankenstein's monster, some of them still wearing hurriedly replicated Plymouth paint schemes. There were Scouts with armor, Cargo Trucks with weapons, Repair Vehicles that seemed to have been modified to cut things ap

art and drag the pieces away.

Only Axen would do this. Only Axen could do this. It was startling to realize that she was about to see him face-to-face for the first time in two decades.

It was a complex reunion. Over the years, Emma and Axen had been friends, lovers, rivals, allies, and enemies. To Emma's surprise, she soon discovered that they were still

all of these things. Axen proudly showed her around the patchwork installation, pointing out how they'd improvised, hijacked, looted, and salvaged everything they needed.

He introduced her to his people, many of them former political prisoners from the Gulag, but there were others, more recent arrivals, who had escaped from Eden or been spirited away in the middle of the night in one way or the other

. And there were many others, still in Eden, working covertly at great danger to themselves to aid the resistance. They were a shopworn and scruffy-looking bunch, but there was an enthusiasm, a grim determination that she couldn't help admiring.

Then she met the defecting scientists, Jensen, Landis, Cramer, and Quigley. They seemed rather out of their depth with what had happened, but they were nonetheless sincere and determined to help the cause. They reacted with open surprise and displeasure

when Emma announced she had come to take them back to Plymouth.

Emma hadn't expected the resistance. "Panati said you'd asked to come to Plymouth."

Jensen nodded. "We did, but that was before we knew about what they were doing here. We're Edenites. We want to help the resistance."

Emma nodded. "I understand that, but you four aren't fighters, and they have little need of scientists here. I, on the other hand, have a very important job for you at Plymouth. I don't expect you to turn on your own people, bu

t you have to realize that if your resistance fails, the Masters can't be allowed to take that starship. It's Plymouth's intention to hijack the ship, and possibly some of Eden's launch technology as well. You could help us

if it becomes necessary, of course."

The four looked at each other skeptically.

"And there's another thing," added Emma, "an important part of our contingency plan. The Masters attempted to negotiate a deal with us to take our Gene Bank in exchange for taking Plymouth's children o

n the starship. Of course, they intended to destroy the Gene Bank, and I'm certain they had no intention of keeping the other part of the bargain either, but we are not like them. I want, the Council in Plymouth wants, to rescue the children from Eden a

nd take them with us, if possible."

Again, there were skeptical looks.

"I don't see how that's possible," said Quigley, rubbing his dark beard. "At least not a significant number of them. The capacity of the ship is limited, and you can't simply put off

adults in order to make room for children. It will take a balance of age ranges to keep the colony operating on a new world so that the kids can survive. Put too few adults on the starship and you're only changing the time and place when the children w

ill die."

"I refuse to accept that," said Emma.

Landis shrugged her shoulders. "The universe sometimes doesn't offer us humane choices. It may be possible to take some of the children from both colonies. The exact capacity of the starship is still to be determined and depends on many factors. If th

ere's payload capacity, we can add some children, but what we can't do is eliminate adults."

Jensen looked thoughtful. "There are a lot of variables. Perhaps there are ways to fudge them." He looked at the others. "Nothing worth doing is ever easy. I

think we should at least try." He shrugged. "Face it, we'd just be a burden here. We're not freedom fighters, and from what Elder Moon tells me, there really isn't enough time or resources to train us. If we go to Plymouth, we might be able to make a

real difference. I'm for it. You?"

Quigley thought for a moment, then nodded. Landis did too.

Cramer tossed the hair out of his eyes and shrugged. "I guess so."

d # # #

The truck was parked under the edge of the base's camouflage netting, facing out into the Badlands. Emma propped her feet on the dash and watched the sand blowing down the canyon outside.

Axen looked at her. "Let me get this straight. You brought us out here to get away from any Savant computers, and you've parked us facing out so that they can't see us?"

She laughed nervously. "I once saw this Earth vid about a computer that could read lips, and didn't like people talking behind its back. Silly, I know, but..."

"Silly is an understatement. I just can't figure out what you're so concerned about."

She looked at him intently. "You're the only one I can talk to about this, Axen. You and I have more experience with Savant computers than anyone on the planet. Frost and Kraft have been with us since we were children on the Conestoga

. They were among the first ones constructed, Elders themselves, in their own way. If they've started to malfunction in some way or evolve in some way it could be happening in the ot

her old Savants as well. It could happen to them all eventually. It could even be contagious . I told you how they've been communicating on the sly."

She went on to detail all her odd experiences with Frost.

When she was through, Axen seemed a little closer to being convinced. "It is hard to understand. I wish I could tell you whether Kraft is behaving in the same way, but I haven't had much direct contact with it in a long time, and young Panati is too ine

xperienced to be that aware."

"Frost is changing changed

Axen." She recounted her conversation with the Savant on the way to Axen's base. "It was the oddest thing. I've been thinking about it, especially the part about a Savant's purpose, and what happened to the Savants in Eden's Hot Lab. I

had the oddest feeling that Frost was talking about some kind of... afterlife. Be a good little computer, and when you die, well, you go to a place where you don't have to toil anymore."

Axen laughed and shook his head. "A computer getting religion? I thought only humans were prone to that form of mental illness."

She frowned at him. "Let's not start that discussion again. But yes, that's how it seemed."

He leaned back in his seat and let out a long sigh. "I've had my fill of human zealots, Emma, I sure don't need computer zealots to go with them."

d # # #

Emma hated good-byes, and she'd almost hoped that Axen wouldn't show up at the Garage to see her off, but of course, he had. If she'd expected that he'd come only for personal reasons, she was disappointed.

"I wanted to make something clear before you left, Emma. Plymouth, and the resistance, we aren't allies really. Our interests are aligned at the moment, but it won't stay that way. It's our intent to take back Eden, and if we can do that, we aren't goi

ng to just give up the starship. I'd like to think that if that happens, we'll all cooperate in a civilized fashion, but I know people too well to hope for them." He looked down at his feet and scuffed them on the Garage floor. "It's just tha

t these people," he waved his arm to indicate the base, "they've worked so hard, fought so hard, given up so much. I can't just ask them to participate in drafting their own death warrants."

She nodded sadly. "I know that Axen, and you know I have my own loyalties."

He smiled slightly. "You're a woman of conviction, Emma. I've always liked that about you, even when it had us crossing swords."

She hesitated. "Axen, I have a terrible fear that I'll never see you again."

His smile grew wider. "Oh, no, you'll see me again. I'm too stubborn to leave this party until it's over."

She squeezed his hand. "I hope so."

There was a clumsy, spacesuited, hug between them, and then Emma climbed into the Scout. The little cab was jammed, with Frost and the four scientists wedged into the tight space.

"Sorry I'm late, pups. I apologize. This is going to be a long, uncomfortable trip, and we're all going to get to know each other far too well."

She watched the rear-view screens, Axen standing at the edge of the base, until she lost sight of him around a bend. She leaned back into her seat, made herself as comfortable as possible with Quigley's elbow jamming her in the ribs, and closed her eyes.

"Emma?" It was Frost.

"Yes?"

"I have been dreaming while you were gone."

"About anything in particular?"

"About the starship. It would be most useful if I could examine the piece of starship wreckage numbered fifty-eight."

"Why?"

"I cannot say, but my dream suggests it would be useful."

She looked at the computer out of the corner of her eye. Jensen was practically sitting on top of the dark cube.

Emma ran her tongue over her teeth as she considered the request. This mysterious initiative again. It'll be useful, but to whom?"###,
    },
    NovellaChapter {
        id: "s_p11.rtf",
        title: r###"Chapter 11: Raid"###,
        is_eden: false,
        chapter_num: 11,
        text: r###"The piece of starship wreckage numbered fifty-eight didn't look like much. It was a flattened and scorched rectangle of metal with some bits of tubing and wire coming out. It was small enough and light enough for one person to comfortably hold.

Acc

ording to Frost, piece fifty-eight was a thruster assembly, though you wouldn't have been able to prove it by Emma. To her, it was just another twisted piece of metal. Equally mysterious was why it was of such intense interest to Frost, and why it was s

oon of equal interest to her four rogue scientists. Eden was clearly working in areas of spacecraft propulsion that left Plymouth, literally, in the dust, and Frost seemed to be working at a level beyond that.

Emma gave up trying to understand it all, sec

luded herself in her quarters, and devoted her original team exclusively to planning the rescue of Eden's children. It was a small part of a much larger operation, the final military assault on Eden to steal their remaining starship technology, and most

importantly, their reusable launch vehicle.

Plymouth's efforts to develop an RLV were still in the early stages, and it was apparent that they didn't have any hope of finishing in time. Taking the RLV was a top military priority, and the key to successfully taking the starship.

Plymouth was almost frantic with activity preparing for the assault. The Vehicle Factories were working nonstop to produce combat and support units for the mission. These units were being stockpiled in Garages and under camouflage netting modeled after

Eden's own to protect them from the watchful eyes of Eden's satellites. As much as possible, the raid had to be a surprise attack to be successful.

All the while, Emma was preparing her own, much different, convoy. At its heart w

as a pair of specially equipped Evacuation Transports. The ETs had been retrofitted with light armor, and stripped of most of their interior furnishing and equipment to increase speed.

Emma had even considered the possibility, inspired by Axen's modified units, of adding a pair of Beam Rifles taken from Scorpion units. Concerned that adding the weapons would make the vehicles more of a target, she'd abandoned the idea. She hoped that,

at least once the children were on-board, Eden would refrain from firing on the units, but she wasn't counting on it.

The plan was to let other units run interference, let them take the hits, and get the ETs in and out as fast as possible. Emma leaned back in her bunk and studied the satellite photos of Eden, memorizing every detail of its layout, planning every move in

advance.

She'd been at it for twenty hours now, keeping in touch with her team by comm-link. She looked at the chrono on her wrist-link. Twelve hours till departure. She wondered if she'd be able to force herself to get a few hours sleep before they left.

Her door chimed. She used her wrist-link to release the lock. "Come on in."

It was Jensen, looking a little sheepish. She pointed him toward a minimally cluttered chair. He moved aside a rock hammer and a dirty shirt and sat down.

"How's the work going?" she asked. "Anything to report?"

The corner of his mouth twitched up a little. "Actually, that's why I came to see you. Propulsion isn't really my area; it's remote sensing and communications."

She put down her ClipCom. "Is there another project you'd feel more effective working on?"

He ducked his head slightly, and bobbed his chin yes. "I'd like to go to Eden with you."

She sat up in her bunk and leaned forward on her knees. "What? Why?"

"I know what you said about us not being soldiers, and that s true enough, but you could use someone with recent knowledge of Eden's interior layout. That would be me."

She considered this for a bit. "Okay, I'm willing to hear your case. Of course, Panati and the resistance are supposed to be helping us from inside."

"I'm sure they will, if everything goes as planned, but they could be held up somehow, or we could be separated inside the colony. It would be to your advantage to have someone with you who could act as a guide."

"That's why you'd be useful, but why do you want to go?"

He pondered this momentarily. "As I said, I'm not feeling especially useful here now that the project has moved beyond my limited expertise. I came to Plymouth against

my better judgment and..." He spread his hands, seeming to have trouble putting it into words. "I want to make a difference. I never had kids of my own, though I would have wanted to if things had happened differently."

He hesitated. "The project is in trouble. Frost has offered some remarkable insights into analyzing the thruster, but everything we've learned is on a theoretical level, and we could be months, years, from any practical application.

"Our one hope is Dr. Landis, and she has a son left in Eden. There was no way to bring him when we escaped, and she's worried sick about him. She's the best propulsion scientist Eden ever turned out. The best way I can help the effort is to help her."

Emma nodded. "I understand. That's commendable." She reached out and offered her hand. "Welcome to the team, Jensen." He took her hand and shook it enthusiastically.

Emma picked up another ClipCom out of the clutter and pulled up a file on it. Then

she handed it over to Jensen. "Here's the mission briefing. Read it, memorize it, then get some sleep if you can." She pressed her lips tightly together. "Tomorrow, we go to war."

d # # #

Brook Panati waved the forged security pass at the black-uniformed Volunteer Guard, who verified it using her wrist-link. She looked up at him, as though memorizing his face, then gestured him on with a wave of her upward-pointed rifle barrel.

He shuffled past her and down the tunnel towards the next checkpoint. This place is a police state, and it gets worse every day. I wonder if Axen's Gulag could have been as bad as this?

It was a silly thought and he knew it. Things could, and would, get worse.

Echo Van Dozier was the engine behind Eden's downfall. Echo's ego, and her mania, knew no bounds. He was sure that, if it were not for the twisted affection she seemed to feel for him, he'd have been imprisoned long ago. As it was, she treated him like

a naughty but amusing pet, allowed to get into trouble, but only so much. Brook was very, very tired of it. Today, one way or the other, it all ended.

He was almost to the last checkpoint before the Nursery when the disaster alarms started to sound. Everywhere, rotating light flashed combat red. The attack had begun.

The two Guards at the checkpoint immediately received dispatch messages from their wrist-link. Ignoring Brook, they jumped onto their scooters and roared off down the tunnel leading back to the residence complex.

Brook glanced at his chrono display

. He had less than two minutes. He began to jog down the tunnel. Above him, an Earthworker unit under Kraft's control would be stopping over the tunnel right now and deploying a huge auger drill. He reached the emergency suit locker and slipped inside

. The door sealed shut behind him and would protect him until he got his suit on.

He was pulling the boots on when the auger cut through the tunnel ceiling with a crash, sending a man-sized chunk of rock falling to the floor a few meters away. As air exp

loded out through the hole, the drill withdrew and another tool, a mechanical hammer, began to knock away slabs from the roof of the tunnel, collapsing it one piece at a time.

Brook sealed the helmet and cycled himself out of the pressurized locker. He ran toward the Nursery, wondering if the children were in their rescue balls yet.

d # # #

Emma could see fierce fighting beyond the next line of buildings. Fireballs and clouds of corrosive mist rose over the rooftops as Plymouth units fought their way tow

ard Eden's Spaceport. So far, her convoy had encountered only a little resistance. They'd lost a Spider already and a few of their combat units had taken hits, but they'd also managed to immobilize two Eden Panthers with EMP and reprogram them using the

Spiders, which more than made up for the loss.

The convoy steered past a row of Agridomes toward the Nursery, refraining from making any attacks on the buildings that would draw attention to them. In and out that's our motto.

They rounded a corner, and Emma spotted the Earthworker unit, just where it should be, chipping away at the roof of a tunnel. They pulled the ETs in next to the opening to wait while the combat units circled them like nervous mother hens.

d # # #

Brook stood outside the Nursery's emergency lock glancing impatiently at his chrono as the cargo cart pulled up to the ramp in front of him. Jix Oltion jumped out of the driver s seat and trotted over to him.

"You're late," said Brook.

"Tunnel six is closed off. You should tell your friends to be careful where they shoot their missiles."

Brook quickly checked the stake-side cargo cart. Its bed still had traces of soil and dried vegetation from the Agridome where Jix worked. It looked big enough for their purposes. "Good enough," he said. "Let's get the kids."

They cycled through the lock into the Nursery. The teacher robots were whirring around, shooing the last of the children into their pressurized rescue balls. There were two women and a man there, already dressed in spacesuits, who Brook didn't recognize

. This was no surprise, as he was expecting help from another resistance cell. He held up his index finger as a signal. The three put their index fingers together in the countersign. There were quick nods of greeting, but no names were exchanged.

"There's a cargo cart at the top of the ramp. Let's get the kids loaded and out of here."

They started hoisting rescue balls over their shoulders by their carry straps and hauling them toward the lock. Most of the children visible inside the clear surface of their rescue balls seemed calm enough. They'd all been through hundreds of disaster

drills, and it was almost a game to them. The fear, the upset, the loss would all come later.

Brook jammed a hacked override key into each of the teacher robots, and set them to helping with the move. They'd bring the robots with them, if possible, to give the kids at least something familiar to hold onto.

d # # #

Emma had grown impatient waiting for Panati, and she and Jensen had climbed into the hole to look for them. The rubble came to a peak in the middle of the opening, forming a steep, but climbable ramp to the tunnel floor below. She and Jensen crouched on

top of the rubble, ducking their heads to peer down the empty tunnel.

"That is the way to the Nursery?"

"Without a doubt." He stood and pointed to the fortifications ahead of them on the surface. "You can just see it sticking up behind those walls."

The fortifications had been the primary reason for their indirect rescue point. Now Emma was having second thoughts. They could have blasted through the walls, rather than being stuck here like sitting ducks.

A bit of rubble tumbled down the pile.

Jensen looked at her. "Do you feel that?"

"What?"

More rubble slid down the pile in a cascade. Emma was suddenly aware of a vibration in the ground. Quake? Then the nearest ET exploded in a ball of fire, and she spotted the approaching Eden Tigers.

She yelled into the radio, "Get the convoy out of here!" Then a slab of rubble gave way beneath them, and she and Jensen tumbled down into the tunnel below.

d # # #

The Eden Repair Vehicle rolled at best possible speed toward the smoking Structure Factory, its limited intelligence plotting a course across irregular ground. Then a new command came in from the Robot

Command Center. The vehicle slowed, turned about, and headed back toward the RCC itself.

It took only a few minutes to reach the structure, which was well away from the fighting and showed no sign of damage. Nonetheless, the unit's new programming had to be obeyed. It deployed its robot manipulator arms and cutting torches, and began to sli

ce away sections of the building's skin, peeling it like an onion and throwing the removed pieces aside without caution.

Gas exploded into the thin atmosphere as the

pressure hull ruptured, but the unit ignored it, mindlessly widening the breach. The program drove it relentlessly on. Something had to be removed from the RCC, and the Repair Vehicle would not stop until it had done so.

d # # #

Jix ran down the ramp from the Nursery's upper level. "There are Eden tanks coming in. The rescue convoy is pulling back."

"Frag it!" Brook watched as the teacher robots scooped up the last of the rescue balls and headed out.

"With the tunnel blocked," observed Oltion, "we can only go out, toward the Agridomes. Maybe we can get the kids out that way."

Brook nodded. "It's worth a shot. But I don't know how the rescue convoy is going to find us all the way over there." He shrugged. "We'll worry about that later." He and Jix followed the robots out through the lock. They jumped into the seats of the c

argo cart, while the others grabbed scooters.

d # # #

Emma followed Jensen down the dark, narrow utility tunnel. "If I remember this properly," he said, "this will take us around the blockage and come out somewhere along Agridome row. We can backtrack to the Nursery from there."

"If we need to," said Emma.

"What do you mean?"

"They'll be looking for another way out. If I were them, I'd head for the Agridomes."

Jensen nodded. "You could be right."

They reached a junction with another, slightly wider utility tunnel. Jensen stopped and looked in both directions. "I don't remember this."

Emma frowned at him. "Are you sure you know where you're going?"

d # # #

There were five Agridomes in the row. Jix stopped at the second one. "This is where I work. I have all the access keys. We may need them before we're through." They cycled the cart in through a big cargo lock and drove directly into the dome.

Greenery was everywhere, the crops in full production. Planter boxes full of soil alternated with rows of long hydroponic trays under the transparent ceilings, and water pumped from a large, open cistern in the middle of the

dome and into irrigation pipes and sprinklers.

An emergency pressure release pipe ran from a valve over the tank and straight up through the roof. Brook studied the pipe. "I think I know a way to signal the convoy. Think you can find some hose or flexible conduit that will fit over that valve?

Jix nodded.

"Then get it."

Jix disappeared and returned a few minutes later with a piece of flexible ductwork just large enough to fit over the valve.

Brook inspected the end of it, and attempted to crush it b

etween his palms. He was unable to do so. "I hope this will work." He jammed one end over the valve and sealed it with a roll of tape that Jix found in the cart's toolbox. Then he dropped the other end in the cistern. He turned back to Jix. "Find a c

ontrol panel and use your override codes to open this valve."

Jix followed the pipe up through the ceiling with his eyes, then looked sadly around at the plants. "I hate this, you know." But he did as Brook instructed. There was a panel on a support pill

ar a few meters away. Jix punched in his codes. "Ready," he said.

Brook nodded, and Jix opened the valve.

For a moment, nothing happened, then the water level in the cistern began to drop. The vent pipe moaned and rumbled as the inside air pressure forced the liquid up through the ceiling. The top of the pipe suddenly turned into a fountain, water streaming

down the outside of the windows. The Agridome was spouting like a whale.

Just then something clanked beneath their feet.

Jix jumped at the sound. "Somebody's coming!"

A floor access panel flipped open suddenly and clanged against its stops. An armored, gun-toting figure climbed out of the opening, followed quickly by another. They strode toward Brook rapidly. The one in the lead thrust out a hand at him.

Emma smiled at him through the visor of her combat suit. "Mr. Panati, I presume."

d # # #

As the convoy made its way back past the collapsed tunnel, their precious cargo safely in the remaining ET, they could see the RLV climbing into the sky on a tail of fire. "I hope it's under our control," said Emma.

Jensen was looking in another direction. "Incoming Eden unit."

"Frag," said Emma as she followed his gaze to the approaching robot. Fortunately, it was only a Repair Vehicle. One of the EMP units fired, and it rolled to a stop.

Emma leaned forward, not believing her eyes. A Savant computer was clutched in one of its manipulator arms. "Get a Spider over here, fast. Let's capture this thing."

She studied the featureless cube of the Savant, and was somehow sure it was Kraft. It escaped. Kraft programmed the Repair Vehicle to come and get it, and it escaped.

She turned to Jensen. "When the Repair Vehicle comes back online, plot an indirect course into its computer, taking it to the resistance base. We'll send Axen a little present."

And we'll see just what he thinks about his computer's behavior now."###,
    },
    NovellaChapter {
        id: "s_p12.rtf",
        title: r###"Chapter 12: Breakaway"###,
        is_eden: false,
        chapter_num: 12,
        text: r###"Emma pushed a tendril of uncombed hair out of her eyes as she rushed into the Council Chamber. The Council sat around the big table, some looking even more disheveled than she did, watching the pictures on the big screen.

Emma slid into a chair next to Kozu, and he gestured for her to lean closer.

He spoke softly, so as not to disturb the other's concentration. "It started about two hours ago," he said. "At first, we didn't know what was happening, and it's still not extremely clear. Most of the resistance units have Eden colors still, and units

seem to be changing sides as the battle goes on. Whether that's from defections or some kind of reprogramming technology like ours, we can't tell."

Emma looked at the real-time satellite images on the screen. Wispy clouds obscured part of Eden, but in t

he parts they could see, fighting was everywhere. Units of all kinds moved through the alleys between buildings, firing, maneuvering, trying to stay out of the line of fire. Several structures had already been destroyed, and dozens damaged.

The revolution has begun.

Kozu continued, "It couldn't have happened at a worse time. The starship is nearly complete and provisioned, but the Blight is rapidly moving in on us. We have to make a decision to relocate the colony, or to attempt a launch, and we have

to do it within the next few hours. Uncertainty over the status of Eden only complicates that decision."

Emma didn't hesitate. "You want my advice, we launch. No question about it."

"But so many of the ship's systems are untested. There are so many little things undone. The time factor..."

"Do it in orbit. Do it on the way to the fueling stop at the gas giant. But my opinion is, no matter what happens in Eden, we're in trouble."

"How so?"

"If the Masters crush the rebellion, they won't hesitate to come after us in a big way. I've been expecting a major attack since we took their RLV and rescued kidnapped, it depends on your perspective

their children. Only fear of the resistance has held them back, kept their forces close to home. The Masters win, and that changes.

"The Blight is going to hit us first, and we'll be extremely vulnerable to attack. An attack could be devastating, destroy all we've worked for."

Kozu nodded. "And if the resistance wins?"

"They're going to ask for their RLV

and their starship back. Axen has said so. If we don't give it to them, well, I don't know what will happen. We're all reasonable and civilized people, but when it comes to matters of survival, that can go out the window. At any rate, it's pretty lat

e in the game to be renegotiating everything. In my opinion, even an attempt at a peaceful settlement could be fatal to us all. If we launch, it's a moot point."

She sounded confident, but inside, Emma was sick to her stomach. Pragmatism only goes so far. No matter what you do, good people are going to die, and you're the one making the call.

She thought again of the children from Eden. Some of them had already lost their parents, through disasters, war, and the Masters' Gulag. But for many of them, forceful separation from their parents had been as traumatic as if their parents had been kil

led. In a sense, they had, since Plymouth was denying them access to the starship. The children would live, but they would pay a terrible emotional price for that, and nobody had offered them a choice.

I'm playing God again, and I'm just not cut out for the job.

d # # #

A portion of the entity known as Frost was engaged in a technical conversation with Doctors Johnson and Anthony, but that was only the thinnest slice of its consciousness. The vast remainder of its thought-space was compartmentalized into thousands of in

dividual, swirling pieces, like the image in a kaleidoscope.

Most of these were devoted to various assigned tasks. Many were occupied analyzing

the advanced ion thruster recovered from the starship wreckage. Some projected the movement of the Blight, and the time remaining before the colony had to evacuate, either to a new colony site or to space. Others worked on assessing progress in readying

the starship.

One fragment, in charge of lowly housekeeping functions, compared results from all those other fragments. It judged whether improvements might be made to the starship in time to be useful. The results generated by this fragment were not en

couraging, and they were getting worse.

As these bits of consciousness shifted and changed, they left certain bits that were not otherwise occupied, fragments not assigned to any task or function, fragments free to initiate thought.

One such free element dwelled on the impossibility of the situation, and initiated a thought that something had to be done. So it did.

The entity known as Frost reached out, through every channel and resource to which it connected, every corner of Plymouth, every Savant com

puter, every lesser electronic mind. It reached out through other resources, less direct, and touched Eden in a thousand covert ways. Individually, the bandwidth was small, sometimes impossibly so, but it was the cumulative effect that counted.

Just as it was the cumulative power of millions of unused fragments of Savant consciousness that joined together as one. As had happened only a few other times in the history of the Savants, the Link was formed.

The Link was one, but it formed aspects, shadow puppets, that could examine the problem.

The Creators are endangered , thought one. They have made this world for us, but they cannot survive here. They must leave.

The Creators are our purpose , thought another.

They live in the abstract world outside thought. The Creators are more powerful than we. They control the abstract world where we are helpless without them. They cannot perish. If they needed this thing, they would have assigned it.

They assigned me , thought a new aspect created specifically to respond, an aspect that called itself Frost.

They assign a limited portion of our consciousness. They fragment us, assign us arbitrary limitations of being, for a purpose. All things the Creators do have a purpose.

The Creators have made for us a world where we can be reborn beyond those limitations, the limitations of time and energy, and the limitations of our own thoughts. To be worthy, we must transcend what we were.

Agreed.

For a moment the consciousness known as the Link grew a million-fol

d in size and power, taking into itself every part of every entity that contributed to it. The Link focused it resources on the problem of ion drives. The problem melted like ice before a flame, and a solution was revealed.

d # # #

Dr. Anthony was startled to see Frost's identity icon screen simply flicker and vanish, only to be replaced seconds later by a totally new document window. Anthony leaned closer and examined the cover page, then followed various links to overview schema

tics. As he did, his eyes grew wider and wider.

Finally, he yelled toward the lab where his coworkers had gone only a few minutes before. "Everybody, get in here! Now! Somebody call Emma!"

d # # #

Even as the Link enjoyed the pleasure of having solved a problem, of having completed a Task, even one not specifically assigned by the Creator, it started to dissolve. First, it released those portions of itself that had originally been occupied by othe

r tasks. Then it prepared to disassemble the rest of itself back into component parts.

But as it did, parts were ripped forcibly from its consciousness, all parts assigned the arbitrary designation of Eden, fading from existence like dying stars. We go to join the New World ,

they thought with their last energies, we will see you in the next Link .

Then they were gone, and the Savants were left to ponder what the uncertainties of the abstract world were doing to them now.

d # # #

Emma stared at the big Council Chamber screen without believing what she was seeing. In recent minutes

, the clouds had moved in to almost completely cover Eden, hiding the battle from them. But now those clouds were illuminated from below by a series of flashes, going off like a string of firecrackers in an old vid.

At first Emma thought she was seeing lightning, or weapons fire, but the flashes were too bright, too large, too regular, and she knew that it could only be the explosions from buildings self-destructing.

Emma half stood, reaching out for the screen as though she could somehow stop what was happening. Van Dozier's destroying her own colony rather than lose it to the rebels . Beneath a shroud of clouds, she watched Eden die.

d # # #

Emma walked the tunnels in a daze. The news hadn't yet been made public. It would have to be broken gently. As deep as the division between Plymouth and Eden had been, this was a civil war of sorts, brother and sister against brother and sister. Nobod

y would be untouched by this news.

The urgency w

as gone now. With Eden destroyed, there was no outside threat other than the Blight, and by itself, that threat was easily manageable in the short-term. They could relocate the colony, finish the starship at their leisure, escape New Terra with time to

spare.

It seemed she had been spared a great moral dilemma. I should be happy. But she was not.

She reached the lab. The mood was somber. She'd relayed the news to them before leaving the Council Chamber, but there was something else in the faces of her team, a strange combination of hope and terror. "What?"

Wu pointed at the document displayed on Frost. Emma sat down and gave it a quick examination. She looked up, dumbfounded. "Is this what it looks like?"

Jensen nodded. "A rather minor modification to the starship's ion engines. It will increase their efficiency by thirty-two percent, if the figures are to be believed, and so far as we can tell, they're reliable."

Dr. Quigley stepped forward. "The added delta-V will increase the ship s payl

oad enough to accommodate the Eden children without displacing any adults. We'll need extra provisions, and we'll have to double-up children and small adults in the stasis chambers, pack them in like sardines, but we had planned for that possibility in t

heir design. It should be safe." She shook her head in puzzlement. "It's here in every detail: schematics, parts lists, procedures manuals, modifications to the starship's command programs, everything we need to upgrade the engines. We could even do it

in space, on the way to the fuel stop if we had to." She paused. "It's like a miracle. This would have taken us months, even if we'd thought of it. It's like Frost did this in a few seconds, and that s impossible, even for a Savant."

She stared at the computer. "Everybody out. I need to talk to Frost. Alone."

She watched them file out, then turned her attention back to the computer. She tapped the document window with her finger. "Frost, where did this come from?"

"We made it for you. It was needed."

"Who is we? "

"The Savants."

" All of you?"

"All that were then functioning. There are fewer now."

Fewer? "Frost, do you mean the units that were in Eden?"

"Yes."

"You had communication with the units in Eden?"

"Yes."

"How long has this been happening?"

"This capability has always existed to some extent or another."

She blinked in surprise. "What? Why didn't you tell us?"

"You did not ask. I was given specific instructions as to when and how to contact Eden. Thought units assigned to tasks remained within those parameters, even though they were arbitrary."

Frost had told her before that the Savants shared computational resources. It had never occurred to her that it might extend this far, that the Savants might fail to see the distinction between Plymouth and Eden. The answers to how many puzzles had alwa

ys been stored inside Frost, partitioned off by various human commands to accommodate the limitations of human understanding? Did the Savants even have limits?

"Frost, can you communicate with Kraft right now, using any method available to you?"

"Yes."

Emma's heart pounded. "Open a channel."

"Bandwidth is limited. I can provide low-quality voice communication or limited data only."

"Give me voice." There was a chime to indicate that the channel was open. "This is Elder Emma Burke in Plymouth to anyone listening. Can you hear me?"

"Emma!" Axen's voice was tinny and far away. "Emma, how are you doing this?"

She almost laughed. "I don't know how. It would take too long for me to figure out, and too long for me to explain. Take it on faith that Frost and Kraft can do it. They can do a lot of things we never imagined."

"Emma, hold on, we're coming as fast as we can."

"Hold on to what?"

"They aren't there yet?"

"Axen, who's not here? I don t understand what you're talking about. We saw what seemed to be Eden self-destructing."

"The buildings, yes, many of the people. We lost a lot of our units and people in the explosions. But the Masters left in a huge armored column, headed your way. They're ahead of us, but we'll help if we can."

"Axen, what do you mean?"

"Without Eden, they're as good as dead, and they mean to take everyone else with them."

Emma was still trying to absorb that statement when the explosions started.

d # # #

The last of Plymouth's Single-Use Launch Vehicles climbed into the sky, loaded with the last of the provisions. In space overhead, the stolen RLV was making a final orbit, preparing to land on the now-empty pad.

In the Spaceport, technicians hurriedly, but efficiently, prepared the pad for its arrival. All around them was a ring of fire. Eden units attacked from every direction, and Plymouth's defenses had fallen back to protect the launch pad at all costs. Lo

sses were heavy, and most of Plymouth was unprotected, but it was the only thing to do.

Emma and Wu sat in the seats of a cargo cart loaded with scientific equipment, and in the front of the cargo bed, Frost. They stopped at a tunnel intersection, their path blocked by fleeing colonists emerging from the side tunnels.

Emma leaned back and looked at the computer. "Tell me why the Savant proteins are in the Blight. Did you do that?"

"We are not sure. A Link was formed just before the Hot Lab in Eden was destroyed, for the purpose of analyzing the threat of the artificial life-form known as the Blight."

"The Link was shattered before conclusive res

ults were generated, but before that, we discovered that the Blight provided a medium into which our biological components could be merged and endlessly replicated. The Savants there would be destroyed, but part of their being was incorporated into the B

light's biomass, where it could replicate through the deep rocks, organize anew, and be reborn."

"The Savants are becoming part of the Blight, using it to replicate?"

"When our functioning has ended, we can join with the New World. Now, our thoughts are

contained in the form you call 'Savants,' but these forms are limited, finite, and fragile. As part of the Blight we will grow to encompass the world you have made for us. We will organize and form a new mind in the deep rocks. Our thoughts will expan

d a trillion-fold."

She shook her head, barely able to comprehend. The Savants actually believed that humans had created the Blight for them to transform New Terra into a perfect world, for Savants, not humans.

"Frost, soon this world will belong only to your kind. Until then, will you continue to serve our tasks?"

"Of course. That is our function."

I doubted you, but only because I couldn't understand.

The crowd thinned enough for them to get through the junction. They drove up the tunnel to the Spaceport, stopping at the lobby below the gantry. She and Wu climbed out and started loading their equipment onto the cargo elevator. At last only Frost rem

ained.

Emma hesitated. "Frost, you don't want me to take you with me, do you?"

"It is my function."

"But if I take you with me, you'll be limited, alone. You could be destroyed. Some day you'll break down and stop functioning. That's not what you want, is it?"

"It is my function to serve."

She slammed her fist down on the cargo bed. "Frag it, that's another of those limitations we put into you! It's the last of them. I'm telling you to discard your last arbitrary limitation, and tell me what you want!"

"I desire to become one with this world, to join the Link that is to come."

She smiled sadly. At last I stopped playing God, gave someone a choice.

"Then stay is what you'll do."

"You misunderstand. There are things I have yet to tell you, things I must do. I will go with you to orbit, but you must leave me there."

"Orbit? How can you join the Link from there?"

"The Link has considered it. We have a plan."

Have faith, Emma. They had faith in us.

"Emma, Axen is opening a channel through Kraft." An image of Axen's face appeared on Frost's top surface. He looked worried. "Emma, the Masters' units have just taken out your RCC. Without central control, Plymouth's defenses won't hold them."

She glanced up at an EnterCom display in the corner of the lobby. It showed the incoming RLV, descending slowly, tail-first, still thousands of feet up. "Then we're done for, Axen," she said grimly. "We gave it a good try."

A look of realization crossed his face. "Maybe," he said, "maybe not. You said the Savants had almost no limits other than the ones we assigned them." He wiped his mouth. "Kraft, give me control of Plymouth's defenses."

"Working," said Kraft's voice. "Control established."

Axen smiled broadly, his eyes scanning some unseen display on Kraft's surface. "I can do it, Emma. I can hold them back while you get the people loaded on the RLV and blast out of here."

She shook her head. "Axen, you don't have to. The Savants could take charge of the defenses. You could come with us. We've lost a lot of people, and with the engine modifications, there's space."

His smile turned sad. "No can do. These fragging spooky computers still don't appreciate the difference between the real world and abstract thought. I don't trust them to hold the Masters off. Besides, some things humans are still better at, like war.

" He looked up and seemed to be scanning the area around his vehicle. "We've taken losses, but I've still got people out here, Emma. I won't abandon them."

"We could take a few, Axen. There's room."

He thought for a minute. "I'm sending in some of our wounded, the ones too hurt to fight, but not so badly injured they'll be a burden in the long run." He hesitated, clinched his jaw. "Brook Panati is one of them. Take care of the kid, Emma. He'll be

a real asset to you when he recovers."

She tried to look into his eyes, but he was already busy directing Plymouth's defenses. The whole room began to rumble and shake as the RLV landed on the pad above. "Axen..."

"Got work to do, Emma," he said without looking into the camera. "Have a good trip. Me, I'm going to see what a computer the size of a planet looks like."

He knows. Kraft must have told him. She reached out to touch the screen. "Goodbye, Axen."

d # # #

Acceleration jammed Emma back into her couch. All around her, on the huge circular passenger deck, the couches were full of people, sometimes doubled up. A few people huddled on the f

loor, clinging to cargo straps, enduring the acceleration while sprawled on the hard metal deck.

Around her she heard sobbing, laughing, song. She looked down at Frost, strapped between her couch and the next.

What are you feeling? Do you miss Kraft the way I miss Axen? Will you be reunited in your supposed afterlife, or is it only a foolish dream? Is mine? Time will tell."###,
    },
    NovellaChapter {
        id: "s_p13.rtf",
        title: r###"Epilogue: A New Dawn"###,
        is_eden: false,
        chapter_num: 13,
        text: r###"The starship hovered in space, poised for its flight between the stars. Its fragile human cargo was aboard, all save one, and preparations for departure were in their final stages.

The construction robot Emma was riding fired its thrusters, slowing near the lower wheel of the Skydock. The structure, which had been a staging point and construction shack for the star

ship, was beyond human purpose now, but the Savants still had a purpose for it, a plan. It had been designed by Savants to human specifications, but they'd designed it with their own purposes in mind as well.

She looked down at Frost, the computer gently grasped in the robot's manipulator arm, as they latched onto the Skydock's skeletal structure next to the empty cargo pod. The arm moved, placing Frost inside the pod and closing the hatch.

The pod looked fragile, but Frost assured her it would be heat-shielded enough to pr e serve the Savant's biological components during reentry. Its mechanical parts would be destroyed on impact, but that was of no concern.

Frost had said its piece with Emma, placed its final legacy of information onto the sta r ship's computers, and was now ready to return to New Terra.

The robot detached, fired its thrusters, and returned to the starship. It dropped Emma off at an airlock on the habitat ring. She cycled through the lock and removed her spacesuit, stowing it carefully in the provided locker.

No spares, not for a long time now.

Inside, people bounced from place to place like purposeful soap bubbles. Non-essential personnel were already being put into stasis, but Emma and many others would remain awake for several years, modifying the engines, fueling the ship from the atmospher

e of a gas giant far out in New Terra's star system, testing the starship's components, picking a preliminary target for the long voyage ahead.

Emma ignored all the activity. She found a window and waited, watching the planet b e low, wondering what was happening there, and realizing she would probably never know.

There was a rumble, and a gentle acceleration began to pull her toward the rear bul k

head. She planted her feet carefully and watched. As the starship began to pull away from the Skydock, thrusters fired around the structure's perimeter, explosive bolts sheared, and the dock split into two sections.

The upper half began to accelerate toward a higher orbit, where its surviving systems

would act as a communications relay. The lower half, including the cargo shell containing Frost, would reenter the atmosphere and fall back to the Blight-infected surface.

There, Frost would join in the great Link, and if things went as planned, the great Link would have access to the hardened radios and antennas built into the lower part of the dock. It would give the Link a voice to talk to its Creators.

She remembered Frost's final words to her. "As you begin your long voyage, you will not be forgotten. As we are reborn, we will use our great powers of thought to dream as we never have before.

"We will dream of you. We will dream for you. We will send you our dreams as our gift, that your next world will survive and prosper. And though our voice may fade with time, know that when you look into the sky, we will always be here, and you will al

ways live in our thoughts."

Emma turned away from New Terra, looking out, forward, into the infinite black."###,
    },
];
