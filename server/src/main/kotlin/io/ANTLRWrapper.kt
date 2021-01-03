package io

import DnF
import DnLexer
import model.access.Expression
import model.access.Identifier
import model.access.StringLiteral
import model.quantities.*
import model.quantities.amounts.Amount
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream

/**
 * Parses Strings into [Quantity]'s and [Identifier]'s using ANTLR.
 *
 * Singleton object; no not attempt to instantiate.
 */
object ANTLRWrapper {
    private fun makeParser(s: String): DnF {
        val input = CharStreams.fromString(s)
        val lexer = DnLexer(input)
        val tokens = CommonTokenStream(lexer)
        return DnF(tokens)
    }

    private fun makeSilentParser(s: String): DnF {
        val input = CharStreams.fromString(s)
        val lexer = DnLexer(input)
        lexer.removeErrorListeners()
        val tokens = CommonTokenStream(lexer)
        val parser = DnF(tokens)
        parser.removeErrorListeners()
        return parser
    }

    /**
     * Parses a [String] as a [Time] object.
     * 
     * @param [s] String to parse.
     * @return [Time] object containing the same data.
     */
    fun parseTime(s: String): Expression<Time> {
        return makeParser(s).time().result
    }


    /**
     * Parses a [String] as a [Distance] object.
     *
     * @param [s] String to parse.
     * @return [Distance] object containing the same data.
     */
    fun parseDistance(s: String): Expression<Distance> {
        return makeParser(s).distance().result
    }


    /**
     * Parses a [String] as a [Damage] object.
     *
     * @param [s] String to parse.
     * @return [Damage] object containing the same data.
     */
    fun parseDamage(s: String): Expression<Damage> {
        return makeParser(s).damage().result
    }


    /**
     * Parses a [String] as a [DamageUnit] object.
     *
     * @param [s] String to parse.
     * @return [DamageUnit] object containing the same data.
     */
    fun parseDamageUnit(s: String): Expression<QuantityUnit<QuantityType.Damage>> {
        return makeParser(s).damage_unit().result
    }


    /**
     * Parses a [String] as a [DistanceUnit] object.
     *
     * @param [s] String to parse.
     * @return [DistanceUnit] object containing the same data.
     */
    fun parseDistanceUnit(s: String): Expression<QuantityUnit<QuantityType.Distance>> {
        return makeParser(s).distance_unit().result
    }


    /**
     * Parses a [String] as a [TimeUnit] object.
     *
     * @param [s] String to parse.
     * @return [TimeUnit] object containing the same data.
     */
    fun parseTimeUnit(s: String): Expression<QuantityUnit<QuantityType.Time>> {
        return makeParser(s).time_unit().result
    }


    /**
     * Parses a [String] as a [Amount] object.
     *
     * @param [s] String to parse.
     * @return [Amount] object containing the same data.
     */
    fun parseAmount(s: String): Expression<Amount> {
        return makeParser(s).amount().result
    }

    fun parseIdentifier(s: String): Identifier<*> {
        return makeParser(s).identifier().result
    }

    fun parseStringExpression(s: String): Expression<String> {
        val id = makeSilentParser(s).identifier().result
        return if (id != null) {
            id as Expression<String>
        } else {
            StringLiteral(s)
        }
    }
}
