# Changelog

## v0.1.0-alpha.1

### New Features

- [`455c392`](https://github.com/noctisynth/merak/commit/455c39232b5ae6704299d6fad511ccec45bf22e5): Add stateful JWT sessions.
- [`7a5e97c`](https://github.com/noctisynth/merak/commit/7a5e97c4a5b4f6b213171f4651a076f610d18bad): Add auth routes for login and register.
- [`1ab7c89`](https://github.com/noctisynth/merak/commit/1ab7c896efe5b1b73ee7ee557f1cd07362113906): Optimize traits for Merak auth module.
- [`3a61ece`](https://github.com/noctisynth/merak/commit/3a61ece9f55faed4fe515fbde043d97ed150b2de): Add `merak-macros` to auto generate database operations.
- [`0677d15`](https://github.com/noctisynth/merak/commit/0677d15083f158af1eaf496d04f244432fbc6869): Implement axum server hello via `utoipa`.
- [`3b53ddc`](https://github.com/noctisynth/merak/commit/3b53ddc9eb4ff196c247203b8bfaa1fa7da9c9fc): Add data model and input model to serialize `surrealdb::RecordId` as utoipa schemas.
- [`d26318a`](https://github.com/noctisynth/merak/commit/d26318a540b93a8b79ab12d4e3d7bdc6dbec43b3): Allow foreign key chain select operations.

### Refactors

- [`a83522e`](https://github.com/noctisynth/merak/commit/a83522e76043b0b5a452becb3b7e0d2f30ac4dfe): Reorganize lib code of merak-macros and move data struct into a feature.
- [`ff28b46`](https://github.com/noctisynth/merak/commit/ff28b46c49e38959415e3c10abfaff4d61536c34): Added common response and error code modules and refactored auth and app entrypoints.
- [`7a5e97c`](https://github.com/noctisynth/merak/commit/7a5e97c4a5b4f6b213171f4651a076f610d18bad): Use Redoc instead of Swagger UI.
- [`8b4bfa8`](https://github.com/noctisynth/merak/commit/8b4bfa89e83d890db1f6632545fd080657264de5): Refactor authentication module to use typed errors with business error codes and standardize API responses with unified error handling.
