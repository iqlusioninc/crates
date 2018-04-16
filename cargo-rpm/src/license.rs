//! Convert Cargo's SPDX 2.1 licenses to the format used by RPM's `License`
//! field. RPM's valid licenses can be found in the "Short Name" column of
//! the "Good Licenses" table from the Licensing page of the Fedora Project Wiki:
//!
//! <https://fedoraproject.org/wiki/Licensing:Main>

#![allow(non_camel_case_types)]

use failure::Error;

/// Convert between an SPDX 2.1 license syntax and the RPM `License` field syntax
pub fn convert(license_string: &str) -> Result<String, Error> {
    let mut output = vec![];

    if license_string.find("/").is_some() {
        // Legacy syntax with '/' delimiter meaning "or"
        for license in license_string.split("/") {
            output.push(License::parse(license)?.as_rpm_str());
        }

        Ok(output.as_slice().join(" or "))
    } else {
        // Preferred syntax with "and"/"or" keywords
        for token in license_string.split_whitespace() {
            output.push(match token.to_lowercase().as_ref() {
                "and" => "and",
                "or" => "or",
                other => License::parse(other)?.as_rpm_str(),
            });
        }

        Ok(output.as_slice().join(" "))
    }
}

/// Licenses for which we have a defined mapping between SPDX 2.1 labels and the
/// RPM `License `field.
pub enum License {
    /// GNU Affero General Public License v3.0 only
    /// <https://spdx.org/licenses/AGPL-3.0-only.html>
    AGPL_3_0_ONLY, // AGPL-3.0-only

    /// GNU Affero General Public License v3.0 or later
    /// <https://spdx.org/licenses/AGPL-3.0-or-later.html>
    AGPL_3_0_OR_LATER, // AGPL-3.0-or-later

    /// Apache License 2.0
    /// <https://spdx.org/licenses/Apache-2.0.html>
    APACHE_2_0, // Apache-2.0

    /// BSD 2-Clause "Simplified" License
    /// <https://spdx.org/licenses/BSD-2-Clause.html>
    BSD_2_CLAUSE, // BSD-2-Clause

    /// BSD 3-Clause "New" or "Revised" License
    /// <https://spdx.org/licenses/BSD-3-Clause.html>
    BSD_3_CLAUSE, // BSD-3-Clause

    /// Creative Commons Zero v1.0 Universal
    /// <https://spdx.org/licenses/CC0-1.0.html>
    CC0_1_0, // CC0-1.0

    /// GNU General Public License v2.0 only
    /// <https://spdx.org/licenses/GPL-2.0-only.html>
    GPL_2_0_ONLY, // GPL-2.0-only

    /// GNU General Public License v2.0 or later
    /// <https://spdx.org/licenses/GPL-2.0-or-later.html>
    GPL_2_0_OR_LATER, // GPL-2.0-or-later

    /// GNU General Public License v3.0 only
    /// <https://spdx.org/licenses/GPL-3.0-only.html>
    GPL_3_0_ONLY, // GPL-3.0-only

    /// GNU General Public License v3.0 or later
    /// <https://spdx.org/licenses/GPL-3.0-or-later.html>
    GPL_3_0_OR_LATER, // GPL-3.0-or-later

    /// GNU Library General Public License v2 only
    /// <https://spdx.org/licenses/LGPL-2.0-only.html>
    LGPL_2_0_ONLY, // LGPL-2.0-only

    /// GNU Library General Public License v2 or later
    /// <https://spdx.org/licenses/LGPL-2.0-or-later.html>
    LGPL_2_0_OR_LATER, // LGPL-2.0-or-later

    /// GNU Lesser General Public License v2.1 only
    /// <https://spdx.org/licenses/LGPL-2.1-only.html>
    LGPL_2_1_ONLY, // LGPL-2.1-only

    /// GNU Lesser General Public License v2.1 or later
    /// <https://spdx.org/licenses/LGPL-2.1-or-later.html>
    LGPL_2_1_OR_LATER, // LGPL-2.1-or-later

    /// GNU Lesser General Public License v3.0 only
    /// <https://spdx.org/licenses/LGPL-3.0-only.html>
    LGPL_3_0_ONLY, // LGPL-3.0-only

    /// GNU Lesser General Public License v3.0 or later
    /// <https://spdx.org/licenses/LGPL-3.0-or-later.html>
    LGPL_3_0_OR_LATER, // LGPL-3.0-or-later

    /// MIT License
    /// <https://spdx.org/licenses/MIT.html>
    MIT, // MIT

    /// Mozilla Public License 2.0
    /// <https://spdx.org/licenses/MPL-2.0.html>
    MPL_2_0, // MPL-2.0
}

impl License {
    /// Parse a specific license into the `License` enum
    pub fn parse(name: &str) -> Result<Self, Error> {
        Ok(match name.to_owned().to_lowercase().as_ref() {
            "agpl-3.0-only" => License::AGPL_3_0_ONLY,
            "agpl-3.0-or-later" => License::AGPL_3_0_OR_LATER,
            "apache-2.0" => License::APACHE_2_0,
            "bsd-2-clause" => License::BSD_2_CLAUSE,
            "bsd-3-clause" => License::BSD_3_CLAUSE,
            "cc0-1.0" => License::CC0_1_0,
            "gpl-2.0-only" => License::GPL_2_0_ONLY,
            "gpl-2.0-or-later" => License::GPL_2_0_OR_LATER,
            "gpl-3.0-only" => License::GPL_3_0_ONLY,
            "gpl-3.0-or-later" => License::GPL_3_0_OR_LATER,
            "lgpl-2.0-only" => License::LGPL_2_0_ONLY,
            "lgpl-2.0-or-later" => License::LGPL_2_0_OR_LATER,
            "lgpl-2.1-only" => License::LGPL_2_1_ONLY,
            "lgpl-2.1-or-later" => License::LGPL_2_1_OR_LATER,
            "lgpl-3.0-only" => License::LGPL_3_0_ONLY,
            "lgpl-3.0-or-later" => License::LGPL_3_0_OR_LATER,
            "mit" => License::MIT,
            "mpl-2.0" => License::MPL_2_0,
            _ => bail!("unknown license: {:?}", name),
        })
    }

    /// Return a license name from the Fedora Licenses table:
    /// <https://fedoraproject.org/wiki/Licensing:Main>
    pub fn as_rpm_str(&self) -> &'static str {
        match *self {
            License::AGPL_3_0_ONLY => "AGPLv3",
            License::AGPL_3_0_OR_LATER => "AGPLv3+",
            License::APACHE_2_0 => "ASL 2.0",
            License::BSD_2_CLAUSE => "BSD",
            License::BSD_3_CLAUSE => "BSD",
            License::CC0_1_0 => "CC0",
            License::GPL_2_0_ONLY => "GPLv2",
            License::GPL_2_0_OR_LATER => "GPLv2+",
            License::GPL_3_0_ONLY => "GPLv3",
            License::GPL_3_0_OR_LATER => "GPLv3+",
            License::LGPL_2_0_ONLY => "LGPLv2",
            License::LGPL_2_0_OR_LATER => "LGPLv2+",
            License::LGPL_2_1_ONLY => "LGPLv2",
            License::LGPL_2_1_OR_LATER => "LGPLv2+",
            License::LGPL_3_0_ONLY => "LGPLv3",
            License::LGPL_3_0_OR_LATER => "LGPLv3+",
            License::MIT => "MIT",
            License::MPL_2_0 => "MPLv2.0",
        }
    }
}
