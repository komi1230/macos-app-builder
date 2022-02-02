// Copyright 2022 Yusuke Kominami
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[allow(non_upper_case_globals)]
pub mod fundamental;
pub mod objc_runtime;

use objc::runtime::Object;

#[link(name = "Foundation", kind = "framework")]
extern "C" {}

pub type Id = *mut Object;

pub struct NSObject(pub Id);

pub struct NSString(pub Id);