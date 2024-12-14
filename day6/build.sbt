val scala3Version = "3.5.2"

lazy val root = project
  .in(file("."))
  .settings(
    name := "day6",
    version := "0.1.0-SNAPSHOT",

    scalaVersion := scala3Version,

    libraryDependencies += "org.scala-lang.modules" %% "scala-parallel-collections" % "1.0.4"
  )
