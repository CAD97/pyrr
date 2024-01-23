# Pyrr Game Engine

<img src="https://github.com/CAD97/pyrr/assets/5992217/ae01bb46-1388-46eb-b31b-1fc417ce0bdf" align="left"></img>
Pyrr is a *highly experimental* and ***incomplete, borderline nonfunctional***
game framework/engine built around the [WASM Component Model].
Engine code is written in Rust using an ECS design similar to [Bevy]'s, but game code is
written in any component model compatible guest language with a familiar object oriented shape.
Pyrr is at the moment a highly incomplete research project unfit for any use, and the author
suggests you use [Bevy], [Fyrox], or [Godot], depending on what you want from your engine.

This README is aimed at developers of the Pyrr engine and tooling itself. Documentation
aimed at downstream development users of the engine will be provided at a later date.

[Bevy]: <https://bevyengine.org/>
[Fyrox]: <https://fyrox.rs/>
[Godot]: <https://godotengine.org/>
[WASM Component Model]: <https://component-model.bytecodealliance.org/>

## Build prerequisites

The only strictly required prerequisite is the most recent Rust toolchain, acquired via [rustup].
Additional optional build scripting support is provided through a [nushell] toolkit: `use toolkit.nu`
and then help should explain what the toolkit can do. The toolkit uses some additional tooling, and
if it isn't available, will prompt you for installation via [cargo-binstall] or manually as needed.

[cargo-binstall]: <https://github.com/cargo-bins/cargo-binstall>
[rustup]: <https://rustup.rs/>
[nushell]: <https://www.nushell.sh/>

## License

This project is licensed under the Mozilla Public License, v. 2.0. (SPDX-License-Identifier: [MPL-2.0])
If a copy of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

Specific libraries in this repository may additionally be available under another license, at your option.

### Contribution

Any contribution submitted by you for inclusion in the covered software shall be licensed as above
(MPL-2.0) unless explicitly stated otherwise.

### Transitive licenses

Additionally, this project consumes upstream libraries licensed under any of

- [Apache-2.0],
- [Apache-2.0] WITH [LLVM-exception], and/or
- [MIT]

which may carry their own attribution requirements. A full list of transitive dependencies is not
directly maintained, and must be acquired by querying `cargo`'s build metadata. The project intends
to limit this list to only [OSI Approved licenses], but *does not* guarantee any accuracy of such.

[Apache-2.0]: <https://spdx.org/licenses/Apache-2.0.html>
[LLVM-exception]: <https://spdx.org/licenses/LLVM-exception.html>
[MIT]: <https://spdx.org/licenses/MIT.html>
[MPL-2.0]: <https://spdx.org/licenses/MPL-2.0.html>
[OSI Approved licenses]: <https://opensource.org/licenses/>

### Disclaimers

> [!CAUTION]
This software is provided by the copyright holders and contributors "AS IS"
and any express or implied warranties, including, but not limited to, the
implied warranties of merchantability and fitness for a particular purpose are
disclaimed. In no event shall the copyright holder or contributors be liable
for any direct, indirect, incidental, special, exemplary, or consequential
damages (including, but not limited to, procurement of substitute goods or
services, loss of use, data, or profits; or business interruption) however
caused and on any theory of liability, whether in contract, strict liability,
or tort (including negligence or otherwise) arising in any way out of the use
of this software, even if advised of the possibility of such damage.

> [!CAUTION]
None of the information provided by this project constitutes legal advice. We strongly advise
consulting a lawyer to understand your obligations when releasing an application which includes
open source licensed software.
