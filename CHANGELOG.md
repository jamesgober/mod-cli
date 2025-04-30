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
            <sup><suP>0.3.8</sup></sup>
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
    <hr><br><br>
<div>



<!-- 0.3.8 | Functional CLI With Styles
============================================
Styles, Colors, Gradients, Themes, Animations, Custom Commands
============================================ -->
## [0.3.8] - 2025-04-29 
<div>
    <h3>Added</h3>
    <ul>
        <li>Added support for terminal colors and text styling options</li>
        <li>Standardized logging color constants (<code>COLOR_SUCCESS</code>, etc.)</li>
        <li>CLI theme management via <code>apply_theme()</code> for terminal foreground/background styling.</li>
        <li>Output hook system (<code>print_info</code>, <code>print_warn</code>, <code>print_error</code>, etc.)</li>
        <li>CLI commands can now display standardized status messages.</li>
        <li>Gradient printing system with RGB interpolation</li>
        <li>Animated CLI progress utilities (progress bar, spinner, percent)</li>
        <li>CLI Input builder (forms, secure prompt, interactive menus).</li>
        <li>Added support for custom commands registration and handling.</li>
        <li>Added <code>Console Mode</code> support via <code>Shell</code>, enabling persistent interactive CLI sessions.</li>
    </ul>
    <h3>Changed</h3>
    <ul>    
        <li>Refactored output into modular format.</li>
        <li>Improved theme system prep with internal constants.</li>
    </ul>
    <hr><br><br>
<div>


<!-- 0.3.0 | Structural Foundation
============================================
Config, plugin system, tables, etc
============================================ -->
## [0.3.0] - 2025-04-13 
<div>
    <h3>Added</h3>
    <ul>
        <li>External plugin system via <code>PluginLoader</code> (behind <code>plugins</code> feature flag)</li>
        <li>CLI output theme engine with support for dark/light/custom styles</li>
        <li>Basic table rendering system (auto-width, adaptive layout)</li>
        <li><code>print_multiline()</code> utility for styled multi-line output with optional delay</li>
        <li>Reading CLI behavior config via <code>config.json</code></li>
        <li>CLI startup banners (defined in config)</li>
        <li>Performance benchmarking support</li>
    </ul>
    <h3>Changed</h3>
    <ul>    
        <li>CLI registry to no longer require mut (technically internal).</li>
    </ul>
    <hr><br><br>
<div>


<!-- 0.2.0 | Command Structure
============================================
Custom commanda, Version, Help, etc.
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
[0.4.0]: https://github.com/jamesgober/mod-cli/compare/v0.3.9...v0.4.0
[0.3.9]: https://github.com/jamesgober/mod-cli/compare/v0.3.8...v0.3.9
[0.3.8]: https://github.com/jamesgober/mod-cli/compare/v0.3.0...v0.3.8
[0.3.0]: https://github.com/jamesgober/mod-cli/compare/v0.2.0...v0.3.0
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