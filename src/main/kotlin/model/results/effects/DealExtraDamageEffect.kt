package model.results.effects

import model.Character
import model.quantities.damage.Damage

class DealExtraDamageEffect(
    val `deal extra damage`: Damage
) : Effect {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}