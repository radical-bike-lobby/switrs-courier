use std::time::SystemTime;

use ckanaction::CKAN;
use serde::Deserialize;

use crate::Error;

#[derive(Deserialize)]
pub struct PackageResponse {
    help: String,
    success: bool,
    result: PackageResult,
}

#[derive(Deserialize)]
pub struct PackageResult {
    count: usize,
    //     "facets": {},
    results: Vec<Package>,
}

#[derive(Deserialize)]
pub struct Package {
    //         "accessLevel": "public",
    accessLevel: String,
    //         "accrualPeriodicity": "R/P1D",
    accrualPeriodicity: String,
    //         "additional_information": "",
    //         "author": "California Highway Patrol",
    author: String,
    //         "author_email": null,
    //         "citation": "",
    //         "conformsTo": "",
    //         "contact_email": "ITSupport@chp.ca.gov",
    contact_email: String,
    //         "contact_name": "California Highway Patrol - Information Technology Division",
    contact_name: String,
    //         "creator_user_id": "b40a3d9a-81c5-4a1e-a4eb-a44af449d81d",
    creator_user_id: String,
    //         "geo_coverage": "",
    //         "granularity": "",
    //         "group": "",
    //         "groups": [],
    //         "id": "80c6a49d-c6b3-40ba-86d8-379c9741b4be",
    id: String,
    //         "isopen": true,
    isopen: bool,
    //         "landingPage": "https://www.chp.ca.gov/",
    landingPage: String,
    //         "language": "",
    //         "license_id": "other-pd",
    license_id: String,
    //         "license_title": "Other (Public Domain)",
    license_title: String,
    //         "maintainer": null,
    //         "maintainer_email": null,
    //         "metadata_created": "2024-05-31T17:59:14.284418",
    #[serde(with = "humantime_serde")]
    metadata_created: SystemTime,
    //         "metadata_modified": "2025-06-10T02:46:05.867066",
    #[serde(with = "humantime_serde")]
    metadata_modified: SystemTime,
    //         "name": "ccrs",
    name: String,
    //         "notes": "Statistic crash records within State of California",
    notes: String,
    //         "num_resources": 31,
    num_resources: usize,
    //         "num_tags": 7,
    num_tags: usize,
    //         "organization": {
    //           "approval_status": "approved",
    //           "created": "2024-05-28T17:52:27.285951",
    //           "description": "The mission of the California Highway Patrol is to provide the highest level of Safety, Service, and Security.",
    //           "id": "788e1aec-ad38-4e68-9eae-d8a9b9a812fc",
    //           "image_url": "2024-05-28-175227.276729LogooftheCaliforniaHighwayPatrol.svg.png",
    //           "is_organization": true,
    //           "name": "california-highway-patrol",
    //           "state": "active",
    //           "title": "California Highway Patrol",
    //           "type": "organization"
    //         },
    //         "owner_org": "788e1aec-ad38-4e68-9eae-d8a9b9a812fc",
    owner_org: String,
    //         "private": false,
    private: bool,
    //         "related_resources": "",
    //         "relationships_as_object": [],
    //         "relationships_as_subject": [],
    resources: Vec<Resource>,
}

#[derive(Deserialize)]
pub struct Resource {
    //             "cache_last_updated": null,
    //             "cache_url": null,
    //             "created": "2024-10-03T17:13:21.055497",
    #[serde(with = "humantime_serde")]
    pub created: SystemTime,
    //             "datastore_active": false,
    pub datastore_active: bool,
    //             "datastore_contains_all_records_of_source_file": false,
    pub datastore_contains_all_records_of_source_file: bool,
    //             "description": "Short description of all data fields in the raw data files.",
    pub description: String,
    //             "format": "DOCX",
    pub format: String,
    //             "hash": "",
    //             "id": "1ec4c13e-b5b6-47df-b794-5107d4eb39ac",
    pub id: String,
    //             "last_modified": "2025-04-04T15:57:03.571269",
    #[serde(with = "humantime_serde")]
    pub last_modified: SystemTime,
    //             "metadata_modified": "2025-04-04T16:02:57.227659",
    #[serde(with = "humantime_serde")]
    pub metadata_modified: SystemTime,
    //             "mimetype": null,
    //             "mimetype_inner": null,
    //             "name": "Raw Data Template",
    pub name: String,
    //             "package_id": "80c6a49d-c6b3-40ba-86d8-379c9741b4be",
    pub package_id: String,
    //             "position": 0,
    //             "resource_type": null,
    //             "size": 44102,
    pub size: Option<usize>,
    //             "state": "active",
    pub state: String,
    //             "url": "https://data.ca.gov/dataset/80c6a49d-c6b3-40ba-86d8-379c9741b4be/resource/1ec4c13e-b5b6-47df-b794-5107d4eb39ac/download/rawdata_template.docx",
    pub url: String,
    //             "url_type": "upload"
    pub url_type: String,
}

/// The Ccrs implementation for fetching data related to the Ccrs
pub struct Ccrs(CKAN);

impl Ccrs {
    /// Construct a new implementation CCRS
    pub fn new() -> Self {
        Self(CKAN::builder().url("https://data.ca.gov").build())
    }

    /// Fetch the CCRS package from CKAN
    pub async fn fetch_ccrs(&self) -> Result<Package, Error> {
        let response = self
            .0
            .package_search()
            .q("ccrs".to_string())
            .call()
            .await
            .map_err(|e| Error::CkanError(e.to_string()))?;
        //println!("{response:#}");
        let response = serde_json::from_value::<PackageResponse>(response)?;

        if !response.success {
            return Err(Error::CkanError(format!("request was not successful")));
        }

        response
            .result
            .results
            .into_iter()
            .find(|p| p.name == "ccrs")
            .ok_or(Error::CkanError(format!("no ccrs package in response")))
    }
}

impl Package {
    /// Lists the resources associated with the CCRS package
    pub fn resources(&self) -> Result<&[Resource], Error> {
        for resource in &self.resources {
            println!("resource name: {}, {}", resource.name, resource.url);
        }

        Ok(&self.resources)
    }
}

#[cfg(test)]
mod tests {
    use crate::calodp::PackageResponse;

    #[test]
    fn test_parse() {
        let data = std::fs::File::open("tests/data/ccrs.json").unwrap();
        let parsed = serde_json::from_reader::<_, PackageResponse>(std::io::BufReader::new(data))
            .expect("Could not parse the Package Data");

        assert!(parsed.success);
        assert_eq!(parsed.result.count, 1);
    }
}
