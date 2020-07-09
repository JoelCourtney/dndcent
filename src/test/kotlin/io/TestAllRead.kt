package io

import io.FileSystemWrapper.getAllContentOfType
import model.races.Race
import model.spells.Spell
import org.junit.jupiter.api.assertDoesNotThrow
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.MethodSource
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.util.stream.Collectors
import java.util.stream.Stream

class TestAllRead {
    @ParameterizedTest(name = "parse spell file {1}")
    @MethodSource
    fun parseSpellFile(dir: String, file: String) {
        assertDoesNotThrow {
            JacksonWrapper.readFile<Spell>(dir, file)
        }
    }

    @ParameterizedTest(name = "parse race file {1}")
    @MethodSource
    fun parseRaceFile(dir: String, file: String) {
        println("why")
        assertDoesNotThrow {
            JacksonWrapper.readFile<Race>(dir, file)
        }
    }

    companion object {
        @JvmStatic
        fun parseSpellFile(): List<Arguments> {
            return getAllContentOfType("Spells").map { Arguments.of(it.parent.toString(), it.fileName.toString()) }
        }

        @JvmStatic
        fun parseRaceFile(): List<Arguments> {
            return getAllContentOfType("Races").map { Arguments.of(it.parent.toString(), it.fileName.toString()) }
        }
    }
}
