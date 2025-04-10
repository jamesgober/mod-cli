<div align="center" id="top">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="./docs/media/jamesgober-logo-dark.png">
        <img  width="72"  height="72" 
            alt="Official brand mark and logo of James Gober. Image shows JG stylish initials encased in a hexagon outline." 
            src="./docs/media/jamesgober-logo.png">
    </picture>
    <h1>
        <strong>CHANGELOG</strong>
        <sup>
            <br><sub>ModCLI</sub><br>
            <sup><suP>0.2.0</sup></sup>
        </sup>
    </h1>
</div>
<!-- 
/////////// END HEADER
#######################################################################################################
/////////// BEGIN BODY -->
<div align="center">
    This document contains a curated, chronologically ordered list of all notable changes for each version and/or release of this project. 
    <br>
    The format of this changelog is based on <a href="https://keepachangelog.com/en/1.1.0/">Keep a Changelog</a>.
    <br><br><br>
</div>

## [Unreleased]
<div>
 <!--
    <h3>Added</h3>
    <ul>
       <li></li>
    </ul>
-->
    <h3>Changed</h3>
    <ul>    
        <li>CLI registry to no longer require mut (technically internal).</li>
    </ul>
    <hr><br><br>
<div>





<!-- 0.2.0
============================================
Initial Commit
============================================-->
## [0.2.0] - 2025-04-10 
<div>
    <h3>Added</h3>
    <ul>
        <li><code>--version</code> flag for app versioning.</li>
        <li>Hidden <code>--modcli</code> flag for framework internals.</li>
        <li>Built-in help command with usage guidance.</li>
        <li>Command argument validation system.</li>
        <li>JSON-based CommandSource loader support.</li>
        <li>Name, alias, and description loading via JSON input</li>
        <li>
            CLI configuration loader with <code>CliConfig</code> struct
            <ul>
                <li>Supports loading CLI settings from a JSON file (<i>examples/config.json</i>)</li>
                <li>
                    Fields include:
                    <ul>
                        <li><code>theme</code>: Optional string to represent CLI theme (e.g., "monochrome", "default").</li>
                        <li><code>strict_args</code>: Optional boolean to enable strict argument checking.</li>
                        <li><code>banner</code>: Optional string to display a CLI welcome message.</li>
                    </ul>
                    <blockquote>Note: Settings are parsed but not yet applied in runtime logic. Behavioral integration will be introduced in a future release.</blockquote>
                </li>
            </ul>
        </li>
        <li>Support for <code>hidden</code> field on commands to hide them from help output.</li>
    </ul>
    <h3>Changed</h3>
    <ul>    
        <li>CLI runner to no longer require mut (technically internal).</li>
    </ul>
    <br>
<div>

<!-- 0.1.0
============================================
Initial Commit
============================================-->
## [0.1.0] - 2025-04-10 
<div>
    <ul>    
        <li>Initial commit &amp; first build.</li>
    </ul>
<div>


<!--
/////////// BEGIN FOOTER
####################################################################################################### -->
[unreleased]: https://github.com/jamesgober/mod-cli/compare/v0.1.0...HEAD
<!-- 
============================================================================
VERSIONS
============================================================================ -->

<!-- 
POST-RELEASE/STABLE GOES HERE
-->


<!--
 PRE-RELEASE =========================================================== -->
[0.1.3]: https://github.com/jamesgober/mod-cli/compare/v0.1.2...v0.1.3
[0.2.0]: https://github.com/jamesgober/mod-cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jamesgober/mod-cli/compare/v0.1.0...HEAD




<!--#######################################################################################################
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>